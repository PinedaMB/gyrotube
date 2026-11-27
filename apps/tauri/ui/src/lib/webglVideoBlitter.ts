/**
 * Minimal GPU-accelerated WebGL video-to-canvas blitter.
 *
 * Avoids `UNPACK_FLIP_Y_WEBGL` (flips Y in the vertex shader instead) so WebKitGTK /
 * ANGLE can take the direct GPU texture upload fast-path instead of a CPU readback.
 * Falls back cleanly to 2D canvas if WebGL is unavailable.
 */
export class WebGLVideoBlitter {
  private readonly canvas: HTMLCanvasElement;
  private gl: WebGLRenderingContext | null = null;
  private texture: WebGLTexture | null = null;
  private ctx2d: CanvasRenderingContext2D | null = null;
  private lastW = 0;
  private lastH = 0;

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    this.initWebGL();
  }

  private initWebGL(): void {
    const opts: WebGLContextAttributes = {
      alpha: false,
      antialias: false,
      depth: false,
      stencil: false,
      preserveDrawingBuffer: false,
      desynchronized: true,
    };
    const gl =
      (this.canvas.getContext("webgl2", opts) as WebGLRenderingContext | null) ??
      (this.canvas.getContext("webgl", opts) as WebGLRenderingContext | null);
    if (!gl) {
      this.ctx2d = this.canvas.getContext("2d", { alpha: false });
      return;
    }

    const vsSrc = `
      attribute vec2 a_pos;
      varying vec2 v_uv;
      void main() {
        gl_Position = vec4(a_pos, 0.0, 1.0);
        v_uv = vec2((a_pos.x + 1.0) * 0.5, (1.0 - a_pos.y) * 0.5);
      }
    `;
    const fsSrc = `
      precision mediump float;
      uniform sampler2D u_tex;
      varying vec2 v_uv;
      void main() {
        gl_FragColor = texture2D(u_tex, v_uv);
      }
    `;

    const compile = (type: number, src: string) => {
      const sh = gl.createShader(type);
      if (!sh) return null;
      gl.shaderSource(sh, src);
      gl.compileShader(sh);
      if (!gl.getShaderParameter(sh, gl.COMPILE_STATUS)) {
        gl.deleteShader(sh);
        return null;
      }
      return sh;
    };

    const vs = compile(gl.VERTEX_SHADER, vsSrc);
    const fs = compile(gl.FRAGMENT_SHADER, fsSrc);
    if (!vs || !fs) {
      this.ctx2d = this.canvas.getContext("2d", { alpha: false });
      return;
    }

    const prog = gl.createProgram();
    if (!prog) {
      this.ctx2d = this.canvas.getContext("2d", { alpha: false });
      return;
    }
    gl.attachShader(prog, vs);
    gl.attachShader(prog, fs);
    gl.linkProgram(prog);
    if (!gl.getProgramParameter(prog, gl.LINK_STATUS)) {
      gl.deleteProgram(prog);
      this.ctx2d = this.canvas.getContext("2d", { alpha: false });
      return;
    }
    gl.useProgram(prog);

    const buf = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, buf);
    gl.bufferData(
      gl.ARRAY_BUFFER,
      new Float32Array([-1, -1, 1, -1, -1, 1, 1, 1]),
      gl.STATIC_DRAW,
    );
    const posLoc = gl.getAttribLocation(prog, "a_pos");
    gl.enableVertexAttribArray(posLoc);
    gl.vertexAttribPointer(posLoc, 2, gl.FLOAT, false, 0, 0);

    const tex = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, tex);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);

    this.gl = gl;
    this.texture = tex;
  }

  public draw(video: HTMLVideoElement, targetW: number, targetH: number): void {
    if (this.canvas.width !== targetW || this.canvas.height !== targetH) {
      this.canvas.width = targetW;
      this.canvas.height = targetH;
      this.lastW = targetW;
      this.lastH = targetH;
      if (this.gl) {
        this.gl.viewport(0, 0, targetW, targetH);
      }
    }

    if (this.gl && this.texture) {
      try {
        const gl = this.gl;
        gl.bindTexture(gl.TEXTURE_2D, this.texture);
        gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, video);
        gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
        return;
      } catch (err) {
        console.warn("[gyro] WebGL video upload failed, falling back:", err);
        this.dispose();
        this.ctx2d = this.canvas.getContext("2d", { alpha: false });
      }
    }

    if (this.ctx2d) {
      this.ctx2d.drawImage(video, 0, 0, this.lastW || targetW, this.lastH || targetH);
    }
  }

  public dispose(): void {
    if (this.gl) {
      if (this.texture) {
        this.gl.deleteTexture(this.texture);
        this.texture = null;
      }
      this.gl = null;
    }
    this.ctx2d = null;
  }
}
