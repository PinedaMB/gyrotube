(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const n of document.querySelectorAll('link[rel="modulepreload"]'))a(n);new MutationObserver(n=>{for(const i of n)if(i.type==="childList")for(const l of i.addedNodes)l.tagName==="LINK"&&l.rel==="modulepreload"&&a(l)}).observe(document,{childList:!0,subtree:!0});function r(n){const i={};return n.integrity&&(i.integrity=n.integrity),n.referrerPolicy&&(i.referrerPolicy=n.referrerPolicy),n.crossOrigin==="use-credentials"?i.credentials="include":n.crossOrigin==="anonymous"?i.credentials="omit":i.credentials="same-origin",i}function a(n){if(n.ep)return;n.ep=!0;const i=r(n);fetch(n.href,i)}})();const yi=!1;var ga=Array.isArray,wi=Array.prototype.indexOf,Br=Array.prototype.includes,ba=Array.from,nn=Object.defineProperty,rr=Object.getOwnPropertyDescriptor,on=Object.getOwnPropertyDescriptors,ki=Object.prototype,$i=Array.prototype,rs=Object.getPrototypeOf,ws=Object.isExtensible;function xr(t){return typeof t=="function"}const yr=()=>{};function Si(t){return t()}function Da(t){for(var e=0;e<t.length;e++)t[e]()}function ln(){var t,e,r=new Promise((a,n)=>{t=a,e=n});return{promise:r,resolve:t,reject:e}}function Ti(t,e){if(Array.isArray(t))return t;if(!(Symbol.iterator in t))return Array.from(t);const r=[];for(const a of t)if(r.push(a),r.length===e)break;return r}const bt=2,Tr=4,Kr=8,as=1<<24,It=16,jt=32,sr=64,Ha=128,ss=256,Ot=512,ot=1024,Ge=2048,Pt=4096,yt=8192,ht=16384,gr=32768,Gr=1<<25,Jt=65536,ia=1<<17,Pi=1<<18,Ir=1<<19,cn=1<<20,Ht=1<<25,oa=1<<21,kr=1<<22,ar=1<<23,Vt=Symbol("$state"),un=Symbol("component"),dn=Symbol("legacy props"),Ai=Symbol(""),vn=Symbol("attributes"),Va=Symbol("class"),Qa=Symbol("style"),Ua=Symbol("text"),sa=Symbol("form reset"),Jr=new class extends Error{name="StaleReactionError";message="The reaction that called `getAbortSignal()` was re-run or destroyed"},ma=!!globalThis.document?.contentType&&globalThis.document.contentType.includes("xml"),Ei=1,Mi=2,fn=4,Ii=8,Ni=16,zi=1,Li=2,hn=4,Ci=8,Ri=16,qi=1,Oi=2,ji=4,Fi=1,Di=2,pn={},nt=Symbol("uninitialized"),_n="http://www.w3.org/1999/xhtml",gn="http://www.w3.org/2000/svg",Hi="http://www.w3.org/1998/Math/MathML",Vi="@attach";function Qi(){console.warn("https://svelte.dev/e/derived_inert")}function Ui(){console.warn("https://svelte.dev/e/select_multiple_invalid_value")}function Wi(){console.warn("https://svelte.dev/e/svelte_boundary_reset_noop")}let bn=!1;function mn(t){return t===this.v}function Bi(t,e){return t!=t?e==e:t!==e||t!==null&&typeof t=="object"||typeof t=="function"}function xn(t){return!Bi(t,this.v)}function Gi(t){throw new Error("https://svelte.dev/e/lifecycle_outside_component")}function Yi(){throw new Error("https://svelte.dev/e/async_derived_orphan")}function Ki(t,e,r){throw new Error("https://svelte.dev/e/each_key_duplicate")}function Ji(t){throw new Error("https://svelte.dev/e/effect_in_teardown")}function Xi(){throw new Error("https://svelte.dev/e/effect_in_unowned_derived")}function Zi(t){throw new Error("https://svelte.dev/e/effect_orphan")}function eo(){throw new Error("https://svelte.dev/e/effect_update_depth_exceeded")}function to(t){throw new Error("https://svelte.dev/e/props_invalid_value")}function ro(){throw new Error("https://svelte.dev/e/state_descriptors_fixed")}function ao(){throw new Error("https://svelte.dev/e/state_prototype_fixed")}function so(){throw new Error("https://svelte.dev/e/state_unsafe_mutation")}function no(){throw new Error("https://svelte.dev/e/svelte_boundary_reset_onerror")}let Nr=!1,io=!1;function oo(){Nr=!0}let Ye=null;function Pr(t){Ye=t}function Je(t,e=!1,r){Ye={p:Ye,i:!1,c:null,e:null,s:t,x:null,r:ke,l:Nr&&!e?{s:null,u:null,$:[]}:null}}function Xe(t){var e=Ye,r=e.e;if(r!==null){e.e=null;for(var a of r)On(a)}return e.i=!0,Ye=e.p,ns(t)}function ns(t={}){return nn(t,un,{value:!0}),t}function zr(){return!Nr||Ye!==null&&Ye.l===null}let vr=[];function yn(){var t=vr;vr=[],Da(t)}function Nt(t){if(vr.length===0&&!Vr){var e=vr;queueMicrotask(()=>{e===vr&&yn()})}vr.push(t)}function lo(){for(;vr.length>0;)yn()}const co=-7169;function rt(t,e){t.f=t.f&co|e}function is(t){(t.f&Ot)!==0||t.deps===null?rt(t,ot):rt(t,Pt)}function wn(t,e){(t.f&Ge)!==0?e.set(t,Ge):(t.f&Pt)!==0&&e.get(t)!==Ge&&e.set(t,Pt),rt(t,ot)}function uo(t,e){if(e){const r=document.body;t.autofocus=!0,Nt(()=>{document.activeElement===r&&t.focus()})}}let ks=!1;function vo(){ks||(ks=!0,document.addEventListener("reset",t=>{Promise.resolve().then(()=>{if(!t.defaultPrevented)for(const e of t.target.elements)e[sa]?.()})},{capture:!0}))}function lr(t){var e=Me,r=ke;zt(null),Lt(null);try{return t()}finally{zt(e),Lt(r)}}function fo(t,e,r,a=r){t.addEventListener(e,()=>lr(r));const n=t[sa];n?t[sa]=()=>{n(),a(!0)}:t[sa]=()=>a(!0),vo()}function kn(t,e,r,a){const n=zr()?Ar:os;var i=t.filter(b=>!b.settled),l=e.map(n);if(r.length===0&&i.length===0){a(l);return}var c=ke,d=ho(),f=i.length===1?i[0].promise:i.length>1?Promise.all(i.map(b=>b.promise)):null;function h(b){if((c.f&ht)===0){d();try{a([...l,...b])}catch(w){Dt(w,c)}la()}}var p=$n();if(r.length===0){f.then(()=>h([])).finally(p);return}function g(){Promise.all(r.map(b=>po(b))).then(h).catch(b=>Dt(b,c)).finally(p)}f?f.then(()=>{if((c.f&ht)!==0){p();return}d(),g(),la()}):g()}function ho(){var t=ke,e=Me,r=Ye,a=we;return function(i=!0){Lt(t),zt(e),Pr(r),i&&(t.f&ht)===0&&(a?.activate(),a?.apply())}}function la(t=!0){Lt(null),zt(null),Pr(null),t&&we?.deactivate()}function $n(){var t=ke,e=t.b,r=we,a=!!e?.is_rendered();return e?.update_pending_count(1,r),r.increment(a,t),()=>{e?.update_pending_count(-1,r),r.decrement(a,t)}}function Ar(t){var e=bt|Ge;return ke!==null&&(ke.f|=Ir),{ctx:Ye,deps:null,effects:null,equals:mn,f:e,fn:t,reactions:null,rv:0,v:nt,wv:0,parent:ke,ac:null}}const Fr=Symbol("obsolete");function po(t,e,r){let a=ke;a===null&&Yi();var n=void 0,i=ir(nt),l=!Me,c=new Set;return Eo(()=>{var d=ke,f=ln();n=f.promise;try{Promise.resolve(t()).then(f.resolve,b=>{b!==Jr&&f.reject(b)}).finally(la)}catch(b){f.reject(b),la()}var h=we;if(l){if((d.f&gr)!==0)var p=$n();if(a.b?.is_rendered())h.async_deriveds.get(d)?.reject(Fr);else for(const b of c.values())b.reject(Fr);c.add(f),h.async_deriveds.set(d,f)}const g=(b,w=void 0)=>{p?.(),c.delete(f),w!==Fr&&(h.activate(),w?(i.f|=ar,Er(i,w)):((i.f&ar)!==0&&(i.f^=ar),Er(i,b)),h.deactivate())};f.promise.then(g,b=>g(null,b||"unknown"))}),ya(()=>{for(const d of c)d.reject(Fr)}),new Promise(d=>{function f(h){function p(){h===n?d(i):f(n)}h.then(p,p)}f(n)})}function Z(t){const e=Ar(t);return Un(e),e}function os(t){const e=Ar(t);return e.equals=xn,e}function _o(t){var e=t.effects;if(e!==null){t.effects=null;for(var r=0;r<e.length;r+=1)pt(e[r])}}function ls(t){var e,r=ke,a=t.parent;if(!Xt&&a!==null&&t.v!==nt&&(a.f&(ht|yt))!==0)return Qi(),t.v;Lt(a);try{_o(t),e=Yn(t)}finally{Lt(r)}return e}function Sn(t){var e=ls(t);if(!t.equals(e)&&(t.wv=Bn(),(!we?.is_fork||t.deps===null)&&(we!==null?(we.capture(t,e,!0),$r?.capture(t,e,!0)):t.v=e,t.deps===null))){rt(t,ot);return}Xt||(Ct!==null?(ds()||we?.is_fork)&&Ct.set(t,e):is(t))}function go(t){if(t.effects!==null)for(const e of t.effects)(e.teardown||e.ac)&&(e.teardown?.(),e.ac!==null&&lr(()=>{e.ac.abort(Jr),e.ac=null}),e.fn!==null&&(e.teardown=yr),Yr(e,0),fs(e))}function Tn(t){if(t.effects!==null)for(const e of t.effects)e.teardown&&e.fn!==null&&Mr(e)}let Ma=null,mr=null,we=null,$r=null,Ct=null,Wa=null,Vr=!1,Ia=!1,Qr=null,na=null;var $s=0;let bo=1;class nr{id=bo++;started=!1;linked=!0;prev=null;next=null;async_deriveds=new Map;current=new Map;previous=new Map;#r=new Set;#l=new Set;#e=0;#i=new Map;#a=null;#t=[];#n=[];#s=new Map;#o=new Map;#f=new Set;is_fork=!1;#c=!1;constructor(){mr===null?Ma=mr=this:(mr.next=this,this.prev=mr),mr=this}#h(){if(this.is_fork)return!0;for(const e of this.#i.keys())if(!this.#g(e))return!0;return!1}#g(e){for(var r=e;r!==null;r=r.parent)if(this.#o.has(r))return!0;return!1}skip_effect(e){this.#o.has(e)||this.#o.set(e,{d:[],m:[]}),this.#f.delete(e)}unskip_effect(e,r=a=>this.schedule(a)){var a=this.#o.get(e);if(a){this.#o.delete(e);for(var n of a.d)rt(n,Ge),r(n);for(n of a.m)rt(n,Pt),r(n)}this.#f.add(e)}#u(){var e=[];for(const i of this.#t)if(!((i.f&ht)!==0||(i.f&(Ge|Pt))===0)){for(var r=i,a=!1;r.parent!==null;){r=r.parent;var n=r.f;if((n&(sr|jt))!==0){if((n&ot)===0){a=!0;break}r.f^=ot}}a||e.push(r)}return this.#t=[],e}#p(){this.started=!0;for(const[c,d]of this.#s)rt(c,d),this.schedule(c);this.apply();for(var e=Qr=[],r=[],a=na=[];this.#t.length>0;){$s++>1e3&&(this.#v(),xo());for(const c of this.#u())try{this.#m(c,e,r)}catch(d){throw En(c),this.#h()||this.discard(),d}if(this.#t.length>0){for(const c of r)this.#t.push(c);for(const c of e)this.#t.push(c);r.length=0,e.length=0}}if(we=null,a.length>0){var n=nr.ensure();for(const c of a)n.schedule(c)}if(Qr=null,na=null,this.#h()){this.#_(r),this.#_(e);for(const[c,d]of this.#o)An(c,d);a.length>0&&we.#p();return}const i=this.#w();if(i){this.#_(r),this.#_(e),i.#d(this);return}this.#s.clear();for(const c of this.#r)c(this);this.#r.clear(),$r=this,Ss(r),Ss(e),$r=null,this.#a?.resolve();var l=we;this.#e===0&&(this.#t.length===0||l!==null)&&this.#v(),l!==null&&(Qt.clear(),l.#p())}#m(e,r,a){e.f^=ot;for(var n=e.first;n!==null;){var i=n.f,l=(i&(jt|sr))!==0,c=l&&(i&ot)!==0,d=c||(i&yt)!==0||this.#o.has(n);if(!d&&n.fn!==null){l?n.f^=ot:(i&Tr)!==0?r.push(n):Zr(n)&&((i&It)!==0&&this.add_dirty_reaction(n,Pt),Mr(n));var f=n.first;if(f!==null){n=f;continue}}for(;n!==null;){var h=n.next;if(h!==null){n=h;break}n=n.parent}}}#w(){for(var e=this.prev;e!==null;e=e.prev)if(!e.is_fork&&this.#b(e))return e;return null}#b(e){for(const[r,[,a]]of this.current)if(!a&&e.current.has(r))return!0;return!1}#d(e){for(const[a,n]of e.current)!this.previous.has(a)&&e.previous.has(a)&&this.previous.set(a,e.previous.get(a)),this.current.set(a,n);for(const[a,n]of e.async_deriveds){const i=this.async_deriveds.get(a);i&&n.promise.then(i.resolve).catch(i.reject)}e.async_deriveds.clear(),this.transfer_reactions(e.#s);const r=a=>{var n=a.reactions;if(n!==null&&!((a.f&bt)!==0&&(a.f&(Ge|Pt))===0))for(const c of n){var i=c.f;if((i&bt)!==0)r(c);else{var l=c;i&(kr|It)&&!this.async_deriveds.has(l)&&(this.#s.delete(l),rt(l,Ge),this.schedule(l))}}};for(const a of this.current.keys())r(a);this.oncommit(()=>e.discard()),e.#v(),we=this,this.#p()}#_(e){for(var r=0;r<e.length;r+=1)wn(e[r],this.#s)}capture(e,r,a=!1){e.v!==nt&&!this.previous.has(e)&&this.previous.set(e,e.v),(e.f&ar)===0&&(this.current.set(e,[r,a]),Ct?.set(e,r)),this.is_fork||(e.v=r)}activate(){we=this}deactivate(){we=null,Ct=null}flush(){try{Ia=!0,we=this,this.#p()}finally{$s=0,Wa=null,Qr=null,na=null,Ia=!1,we=null,Ct=null,Qt.clear()}}discard(){for(const e of this.#l)e(this);this.#l.clear();for(const e of this.async_deriveds.values())e.reject(Fr);this.#v(),this.#a?.resolve()}register_created_effect(e){this.#n.push(e)}#x(){for(let p=Ma;p!==null;p=p.next){var e=p.id<this.id,r=[];for(const[g,[b,w]]of this.current){if(p.current.has(g)){var a=p.current.get(g)[0];if(e&&b!==a)p.current.set(g,[b,w]);else continue}r.push(g)}if(e)for(const[g,b]of this.async_deriveds){const w=p.async_deriveds.get(g);w&&b.promise.then(w.resolve).catch(w.reject)}var n=[...p.current.keys()].filter(g=>!p.current.get(g)[1]);if(!(!p.started||n.length===0)){var i=n.filter(g=>!this.current.has(g));if(i.length===0)e&&p.discard();else if(r.length>0){if(e)for(const g of this.#f)p.unskip_effect(g,b=>{(b.f&(It|kr))!==0?p.schedule(b):p.#_([b])});p.activate();var l=new Set,c=new Map;for(var d of r)Pn(d,i,l,c);c=new Map;var f=[...p.current].filter(([g,b])=>{const w=this.current.get(g);return w?w[0]!==b[0]||w[1]!==b[1]:!0}).map(([g])=>g);if(f.length>0)for(const g of this.#n)(g.f&(ht|yt|ia))===0&&cs(g,f,c)&&((g.f&(kr|It))!==0?(rt(g,Ge),p.schedule(g)):p.add_dirty_reaction(g,Ge));if(p.#t.length>0&&!p.#c){p.apply();for(var h of p.#u())p.#m(h,[],[])}p.deactivate()}}}}increment(e,r){if(this.#e+=1,e){let a=this.#i.get(r)??0;this.#i.set(r,a+1)}}decrement(e,r){if(this.#e-=1,e){let a=this.#i.get(r)??0;a===1?this.#i.delete(r):this.#i.set(r,a-1)}this.#c||(this.#c=!0,Nt(()=>{this.#c=!1,this.linked&&this.flush()}))}transfer_reactions(e){for(const[r,a]of e)this.add_dirty_reaction(r,a);e.clear()}add_dirty_reaction(e,r){(r===Ge||this.#s.get(e)!==Ge)&&this.#s.set(e,r)}oncommit(e){this.#r.add(e)}ondiscard(e){this.#l.add(e)}settled(){return(this.#a??=ln()).promise}static ensure(){if(we===null){const e=we=new nr;!Ia&&!Vr&&Nt(()=>{e.started||e.flush()})}return we}apply(){{Ct=null;return}}schedule(e){if(Wa=e,e.b?.is_pending&&(e.f&(Tr|Kr|as))!==0&&(e.f&gr)===0){e.b.defer_effect(e);return}this.#t.push(e)}#v(){if(this.linked){var e=this.prev,r=this.next;e===null?Ma=r:e.next=r,r===null?mr=e:r.prev=e,this.linked=!1}}}function mo(t){var e=Vr,r=$r;$r=null,Vr=!0;try{for(var a;;){if(lo(),we===null)return a;we.flush()}}finally{Vr=e,$r=r}}function xo(){try{eo()}catch(t){Dt(t,Wa)}}let Yt=null;function Ss(t){var e=t.length;if(e!==0){for(var r=0;r<e;){var a=t[r++];if((a.f&(ht|yt))===0&&Zr(a)&&(Yt=new Set,Mr(a),a.deps===null&&a.first===null&&a.nodes===null&&a.teardown===null&&a.ac===null&&Hn(a),Yt?.size>0)){Qt.clear();for(const n of Yt){if((n.f&(ht|yt))!==0)continue;const i=[n];let l=n.parent;for(;l!==null;)Yt.has(l)&&(Yt.delete(l),i.push(l)),l=l.parent;for(let c=i.length-1;c>=0;c--){const d=i[c];(d.f&(ht|yt))===0&&Mr(d)}}Yt.clear()}}Yt=null}}function Pn(t,e,r,a){if(!r.has(t)&&(r.add(t),t.reactions!==null))for(const n of t.reactions){const i=n.f;(i&bt)!==0?Pn(n,e,r,a):(i&(kr|It))!==0&&(i&Ge)===0&&cs(n,e,a)&&(rt(n,Ge),us(n))}}function cs(t,e,r){const a=r.get(t);if(a!==void 0)return a;if(t.deps!==null)for(const n of t.deps){if(Br.call(e,n))return!0;if((n.f&bt)!==0&&cs(n,e,r))return r.set(n,!0),!0}return r.set(t,!1),!1}function us(t){we.schedule(t)}function An(t,e){if(!((t.f&jt)!==0&&(t.f&ot)!==0)){(t.f&Ge)!==0?e.d.push(t):(t.f&Pt)!==0&&e.m.push(t),rt(t,ot);for(var r=t.first;r!==null;)An(r,e),r=r.next}}function En(t){rt(t,ot);for(var e=t.first;e!==null;)En(e),e=e.next}let ca=new Set;const Qt=new Map;let Mn=!1;function ir(t,e){var r={f:0,v:t,reactions:null,equals:mn,rv:0,wv:0};return r}function O(t,e){const r=ir(t);return Un(r),r}function yo(t,e=!1,r=!0){const a=ir(t);return e||(a.equals=xn),Nr&&r&&Ye!==null&&Ye.l!==null&&(Ye.l.s??=[]).push(a),a}function x(t,e,r=!1){Me!==null&&(!qt||(Me.f&ia)!==0)&&zr()&&(Me.f&(bt|It|kr|ia))!==0&&(Wt===null||!Wt.has(t))&&so();let a=r?Ke(e):e;return Er(t,a,na)}var dr=null,Ba=0;function Er(t,e,r=null){if(!t.equals(e)){Xt?Qt.set(t,e):Qt.has(t)||Qt.set(t,t.v);var a=nr.ensure();if(a.capture(t,e),(t.f&bt)!==0){const n=t;(t.f&Ge)!==0&&ls(n),Ct===null&&is(n)}t.wv=Bn(),dr=null,Ba=0,In(t,Ge,r),dr=null,zr()&&ke!==null&&(ke.f&ot)!==0&&(ke.f&(jt|sr))===0&&(Et===null?Io([t]):Et.push(t)),!a.is_fork&&ca.size>0&&!Mn&&wo()}return e}function wo(){Mn=!1;for(const t of ca){(t.f&ot)!==0&&rt(t,Pt);let e;try{e=Zr(t)}catch{e=!0}e&&Mr(t)}ca.clear()}function Ts(t,e=1){var r=s(t),a=e===1?r++:r--;return x(t,r),a}function Ur(t){x(t,t.v+1)}function In(t,e,r){var a=t.reactions;if(a!==null){var n=zr(),i=a.length;if(Ba+=i,Ba>1e5&&dr===null&&(dr=new Set),dr!==null){if(dr.has(t))return;dr.add(t)}for(var l=0;l<i;l++){var c=a[l],d=c.f;if(!(!n&&c===ke)){var f=(d&Ge)===0;if(f&&rt(c,e),(d&ia)!==0)ca.add(c);else if((d&bt)!==0){var h=c;Ct?.delete(h),In(h,Pt,r)}else if(f){var p=c;(d&It)!==0&&Yt!==null&&Yt.add(p),r!==null?r.push(p):us(p)}}}}}function Ke(t){if(typeof t!="object"||t===null||Vt in t||un in t)return t;const e=rs(t);if(e!==ki&&e!==$i)return t;var r=new Map,a=ga(t),n=O(0),i=pr,l=c=>{if(pr===i)return c();var d=Me,f=pr;zt(null),Ms(i);var h=c();return zt(d),Ms(f),h};return a&&r.set("length",O(t.length)),new Proxy(t,{defineProperty(c,d,f){(!("value"in f)||f.configurable===!1||f.enumerable===!1||f.writable===!1)&&ro();var h=r.get(d);return h===void 0?l(()=>{var p=O(f.value);return r.set(d,p),p}):x(h,f.value,!0),!0},deleteProperty(c,d){var f=r.get(d);if(f===void 0){if(d in c){const h=l(()=>O(nt));r.set(d,h),Ur(n)}}else x(f,nt),Ur(n);return!0},get(c,d,f){if(d===Vt)return t;var h=r.get(d),p=d in c;if(h===void 0&&(!p||rr(c,d)?.writable)&&(h=l(()=>{var b=Ke(p?c[d]:nt),w=O(b);return w}),r.set(d,h)),h!==void 0){var g=s(h);return g===nt?void 0:g}return Reflect.get(c,d,f)},getOwnPropertyDescriptor(c,d){this.has?.(c,d);var f=Reflect.getOwnPropertyDescriptor(c,d),h=r.get(d);if(h!==void 0){var p=s(h);if(p===nt)return;if(f&&"value"in f)f.value=p;else return{enumerable:!0,configurable:!0,value:p,writable:!0}}return f},has(c,d){if(d===Vt)return!0;var f=r.get(d),h=f!==void 0&&f.v!==nt||Reflect.has(c,d);if(f!==void 0||(ke!==null||Me!==null)&&(!h||rr(c,d)?.writable)){f===void 0&&(f=l(()=>{var g=h?Ke(c[d]):nt,b=O(g);return b}),r.set(d,f));var p=s(f);if(p===nt)return!1}return h},set(c,d,f,h){var p=r.get(d),g=d in c;if(a&&d==="length")for(var b=f;b<p.v;b+=1){var w=r.get(b+"");w!==void 0?x(w,nt):b in c&&(w=l(()=>O(nt)),r.set(b+"",w))}if(p===void 0)(!g||rr(c,d)?.writable)&&(p=l(()=>O(void 0)),x(p,Ke(f)),r.set(d,p));else{g=p.v!==nt;var L=l(()=>Ke(f));x(p,L)}var k=Reflect.getOwnPropertyDescriptor(c,d);if(k?.set&&k.set.call(h,f),!g){if(a&&typeof d=="string"){var M=r.get("length"),A=Number(d);Number.isInteger(A)&&A>=M.v&&x(M,A+1)}Ur(n)}return!0},ownKeys(c){s(n);var d=Reflect.ownKeys(c).filter(p=>{var g=r.get(p);return g===void 0||g.v!==nt});for(var[f,h]of r)h.v!==nt&&!(f in c)&&d.push(f);return d},setPrototypeOf(){ao()}})}function Ps(t){try{if(t!==null&&typeof t=="object"&&Vt in t)return t[Vt]}catch{}return t}function Nn(t,e){return Object.is(Ps(t),Ps(e))}var As,zn,Ln,Cn;function ko(){if(As===void 0){As=window,zn=/Firefox/.test(navigator.userAgent);var t=Element.prototype,e=Node.prototype,r=Text.prototype;Ln=rr(e,"firstChild").get,Cn=rr(e,"nextSibling").get,ws(t)&&(t[Va]=void 0,t[vn]=null,t[Qa]=void 0,t.__e=void 0),ws(r)&&(r[Ua]=void 0)}}function Ut(t=""){return document.createTextNode(t)}function Rt(t){return Ln.call(t)}function Xr(t){return Cn.call(t)}function v(t,e){return Rt(t)}function se(t,e=!1){{var r=Rt(t);return r instanceof Comment&&r.data===""?Xr(r):r}}function N(t,e=!1){return Rt(t)}function u(t,e=1,r=!1){let a=t;for(;e--;)a=Xr(a);return a}function $o(t){t.textContent=""}function Rn(){return!1}function xa(t,e,r){return e==null||e===_n?r?document.createElement(t,{is:r}):document.createElement(t):r?document.createElementNS(e,t,{is:r}):document.createElementNS(e,t)}function So(t){var e=ke;if(e===null)return Me.f|=ar,t;if((e.f&gr)===0&&(e.f&Tr)===0)throw t;Dt(t,e)}function Dt(t,e){if(t===pn)throw t;if(!(e!==null&&(e.f&ht)!==0)){for(;e!==null;){if((e.f&Ha)!==0&&(e.f&(ht|Gr))===0){if((e.f&gr)===0)throw t;try{e.b.error(t);return}catch(r){t=r}}e=e.parent}throw t}}function qn(t){ke===null&&(Me===null&&Zi(),Xi()),Xt&&Ji()}function To(t,e){var r=e.last;r===null?e.last=e.first=t:(r.next=t,t.prev=r,e.last=t)}function Ft(t,e){var r=ke;r!==null&&(r.f&yt)!==0&&(t|=yt);var a={ctx:Ye,deps:null,nodes:null,f:t|Ge|Ot,first:null,fn:e,last:null,next:null,parent:r,b:r&&r.b,prev:null,teardown:null,wv:0,ac:null};we?.register_created_effect(a);var n=a;if((t&Tr)!==0)Qr!==null?Qr.push(a):nr.ensure().schedule(a);else if(e!==null){try{Mr(a)}catch(l){throw pt(a),l}n.deps===null&&n.teardown===null&&n.nodes===null&&n.first===n.last&&(n.f&Ir)===0&&(n=n.first,(t&It)!==0&&(t&Jt)!==0&&n!==null&&(n.f|=Jt))}if(n!==null&&(n.parent=r,r!==null&&To(n,r),Me!==null&&(Me.f&bt)!==0&&(t&sr)===0)){var i=Me;(i.effects??=[]).push(n)}return a}function ds(){return Me!==null&&!qt}function ya(t){const e=Ft(Kr,null);return rt(e,ot),e.teardown=t,e}function At(t){qn();var e=ke.f,r=!Me&&(e&jt)!==0&&Ye!==null&&!Ye.i;if(r){var a=Ye;(a.e??=[]).push(t)}else return On(t)}function On(t){return Ft(Tr|cn,t)}function Po(t){return qn(),Ft(Kr|cn,t)}function Ao(t){nr.ensure();const e=Ft(sr|Ir,t);return(r={})=>new Promise(a=>{r.outro?hr(e,()=>{pt(e),a(void 0)}):(pt(e),a(void 0))})}function wa(t){return Ft(Tr,t)}function Eo(t){return Ft(kr|Ir,t)}function vs(t,e=0){return Ft(Kr|e,t)}function U(t,e=[],r=[],a=[]){kn(a,e,r,n=>{Ft(Kr,()=>{t(...n.map(s))})})}function Lr(t,e=0){var r=Ft(It|e,t);return r}function jn(t,e=0){var r=Ft(as|e,t);return r}function kt(t){return Ft(jt|Ir,t)}function Fn(t){var e=t.teardown;if(e!==null){const r=Xt,a=Me;Es(!0),zt(null);try{e.call(null)}catch(n){Dt(n,t.parent)}finally{Es(r),zt(a)}}}function fs(t,e=!1){var r=t.first;for(t.first=t.last=null;r!==null;){const n=r.ac;n!==null&&lr(()=>{n.abort(Jr)});var a=r.next;(r.f&sr)!==0?r.parent=null:pt(r,e),r=a}}function Mo(t){for(var e=t.first;e!==null;){var r=e.next;(e.f&jt)===0&&pt(e),e=r}}function pt(t,e=!0){var r=!1;(e||(t.f&Pi)!==0)&&t.nodes!==null&&t.nodes.end!==null&&(Dn(t.nodes.start,t.nodes.end),r=!0),t.f|=Gr,fs(t,e&&!r),Yr(t,0);var a=t.nodes&&t.nodes.t;if(a!==null)for(const i of a)i.stop();Fn(t),t.f^=Gr,t.f|=ht;var n=t.parent;n!==null&&n.first!==null&&Hn(t),t.next=t.prev=t.teardown=t.ctx=t.deps=t.fn=t.nodes=t.ac=t.b=null}function Dn(t,e){for(;t!==null;){var r=t===e?null:Xr(t);t.remove(),t=r}}function Hn(t){var e=t.parent,r=t.prev,a=t.next;r!==null&&(r.next=a),a!==null&&(a.prev=r),e!==null&&(e.first===t&&(e.first=a),e.last===t&&(e.last=r))}function hr(t,e,r=!0){var a=[];t.f|=ss,Vn(t,a,!0);var n=()=>{r&&pt(t),e&&e()},i=a.length;if(i>0){var l=()=>--i||n();for(var c of a)c.out(l)}else n()}function Vn(t,e,r){if((t.f&yt)===0){t.f^=yt;var a=t.nodes&&t.nodes.t;if(a!==null)for(const c of a)(c.is_global||r)&&e.push(c);for(var n=t.first;n!==null;){var i=n.next;if((n.f&sr)===0){var l=(n.f&Jt)!==0||(n.f&jt)!==0&&(t.f&It)!==0;Vn(n,e,l?r:!1)}n=i}}}function ua(t){t.f&=~ss,Qn(t,!0)}function Qn(t,e){if((t.f&ss)===0&&(t.f&yt)!==0){t.f^=yt,(t.f&ot)===0&&(rt(t,Ge),nr.ensure().schedule(t));for(var r=t.first;r!==null;){var a=r.next,n=(r.f&Jt)!==0||(r.f&jt)!==0;Qn(r,n?e:!1),r=a}var i=t.nodes&&t.nodes.t;if(i!==null)for(const l of i)(l.is_global||e)&&l.in()}}function hs(t,e){if(t.nodes)for(var r=t.nodes.start,a=t.nodes.end;r!==null;){var n=r===a?null:Xr(r);e.append(r),r=n}}let Xt=!1;function Es(t){Xt=t}let Me=null,qt=!1;function zt(t){Me=t}let ke=null;function Lt(t){ke=t}let Wt=null;function Un(t){Me!==null&&((Me.f&oa)!==0||(Me.f&bt)!==0)&&(Wt??=new Set).add(t)}let $t=null,Tt=0,Et=null;function Io(t){Et=t}let Wn=1,fr=0,pr=fr;function Ms(t){pr=t}function Bn(){return++Wn}function Zr(t){var e=t.f;if((e&Ge)!==0)return!0;if((e&Pt)!==0){for(var r=t.deps,a=r.length,n=0;n<a;n++){var i=r[n];if(Zr(i)&&Sn(i),i.wv>t.wv)return!0}(e&Ot)!==0&&Ct===null&&rt(t,ot)}return!1}function Gn(t,e,r=!0){var a=t.reactions;if(a!==null&&!(Wt!==null&&Wt.has(t)))for(var n=0;n<a.length;n++){var i=a[n];(i.f&bt)!==0?Gn(i,e,!1):e===i&&(r?rt(i,Ge):(i.f&ot)!==0&&rt(i,Pt),us(i))}}function Yn(t){var e=$t,r=Tt,a=Et,n=Me,i=Wt,l=Ye,c=qt,d=pr,f=t.f;$t=null,Tt=0,Et=null,Me=(f&(jt|sr))===0?t:null,Wt=null,Pr(t.ctx),qt=!1,pr=++fr,t.ac!==null&&(lr(()=>{t.ac.abort(Jr)}),t.ac=null);try{t.f|=oa;var h=t.fn,p=h();t.f|=gr;var g=Is(t);if(zr()&&Et!==null&&!qt&&g!==null&&(t.f&(bt|Pt|Ge))===0)for(var b=0;b<Et.length;b++)Gn(Et[b],t);if(n!==null&&n!==t){if(fr++,n.deps!==null)for(let w=0;w<r;w+=1)n.deps[w].rv=fr;if(e!==null)for(const w of e)w.rv=fr;Et!==null&&(a===null?a=Et:a.push(...Et))}return(t.f&ar)!==0&&(t.f^=ar),p}catch(w){return Is(t),So(w)}finally{t.f^=oa,$t=e,Tt=r,Et=a,Me=n,Wt=i,Pr(l),qt=c,pr=d}}function Is(t){var e=t.deps,r=we?.is_fork;if($t!==null){var a;if(r||Yr(t,Tt),e!==null&&Tt>0)for(e.length=Tt+$t.length,a=0;a<$t.length;a++)e[Tt+a]=$t[a];else t.deps=e=$t;if(ds()&&(t.f&Ot)!==0)for(a=Tt;a<e.length;a++)(e[a].reactions??=[]).push(t)}else!r&&e!==null&&Tt<e.length&&(Yr(t,Tt),e.length=Tt);return e}function No(t,e){let r=e.reactions;if(r!==null){var a=wi.call(r,t);if(a!==-1){var n=r.length-1;n===0?r=e.reactions=null:(r[a]=r[n],r.pop())}}if(r===null&&(e.f&bt)!==0&&($t===null||!Br.call($t,e))){var i=e;(i.f&Ot)!==0&&(i.f^=Ot),i.v!==nt&&is(i),i.ac!==null&&lr(()=>{i.ac.abort(Jr),i.ac=null,rt(i,Ge)}),go(i),Yr(i,0)}}function Yr(t,e){var r=t.deps;if(r!==null)for(var a=e;a<r.length;a++)No(t,r[a])}function Mr(t){var e=t.f;if((e&ht)===0){rt(t,ot);var r=ke;ke=t;try{(e&(It|as))!==0?Mo(t):fs(t),Fn(t);var a=Yn(t);t.teardown=typeof a=="function"?a:null,t.wv=Wn;var n;yi&&io&&(t.f&Ge)!==0&&t.deps}finally{ke=r}}}async function zo(){await Promise.resolve(),mo()}function s(t){var e=t.f,r=(e&bt)!==0;if(Me!==null&&!qt){var a=ke!==null&&(ke.f&ht)!==0;if(!a&&(Wt===null||!Wt.has(t))){var n=Me.deps;if((Me.f&oa)!==0)t.rv<fr&&(t.rv=fr,$t===null&&n!==null&&n[Tt]===t?Tt++:$t===null?$t=[t]:$t.push(t));else{Me.deps??=[],Br.call(Me.deps,t)||Me.deps.push(t);var i=t.reactions;i===null?t.reactions=[Me]:Br.call(i,Me)||i.push(Me)}}}if(Xt&&Qt.has(t))return Qt.get(t);if(r){var l=t;if(Xt){var c=l.v;return((l.f&ot)===0&&l.reactions!==null||Jn(l))&&(c=ls(l)),Qt.set(l,c),c}var d=(l.f&Ot)===0&&!qt&&Me!==null&&(Me.f&Ot)!==0,f=(l.f&gr)===0;Zr(l)&&(d&&(l.f|=Ot),Sn(l)),d&&!f&&(Tn(l),Kn(l))}if(Ct?.has(t))return Ct.get(t);if((t.f&ar)!==0)throw t.v;return t.v}function Kn(t){if(t.f|=Ot,t.deps!==null)for(const r of t.deps){var e=r.reactions;e===null?r.reactions=[t]:Br.call(e,t)||e.push(t),(r.f&bt)!==0&&(r.f&Ot)===0&&(Tn(r),Kn(r))}}function Jn(t){if(t.v===nt)return!0;if(t.deps===null)return!1;for(const e of t.deps)if(Qt.has(e)||(e.f&bt)!==0&&Jn(e))return!0;return!1}function Zt(t){var e=qt;try{return qt=!0,t()}finally{qt=e}}function ur(t){if(!(typeof t!="object"||!t||t instanceof EventTarget)){if(Vt in t)Ga(t);else if(!Array.isArray(t))for(let e in t){const r=t[e];typeof r=="object"&&r&&Vt in r&&Ga(r)}}}function Ga(t,e=new Set){if(typeof t=="object"&&t!==null&&!(t instanceof EventTarget)&&!e.has(t)){e.add(t),t instanceof Date&&t.getTime();for(let a in t)try{Ga(t[a],e)}catch{}const r=rs(t);if(r!==Object.prototype&&r!==Array.prototype&&r!==Map.prototype&&r!==Set.prototype&&r!==Date.prototype){const a=on(r);for(let n in a){const i=a[n].get;if(i)try{i.call(t)}catch{}}}}}function Lo(t){return t.endsWith("capture")&&t!=="gotpointercapture"&&t!=="lostpointercapture"}const Co=["beforeinput","click","change","dblclick","contextmenu","focusin","focusout","input","keydown","keyup","mousedown","mousemove","mouseout","mouseover","mouseup","pointerdown","pointermove","pointerout","pointerover","pointerup","touchend","touchmove","touchstart"];function Ro(t){return Co.includes(t)}const qo={formnovalidate:"formNoValidate",ismap:"isMap",nomodule:"noModule",playsinline:"playsInline",readonly:"readOnly",defaultvalue:"defaultValue",defaultchecked:"defaultChecked",srcobject:"srcObject",novalidate:"noValidate",allowfullscreen:"allowFullscreen",disablepictureinpicture:"disablePictureInPicture",disableremoteplayback:"disableRemotePlayback"};function Oo(t){return t=t.toLowerCase(),qo[t]??t}const jo=["touchstart","touchmove"];function Fo(t){return jo.includes(t)}const Dr=Symbol("events"),Xn=new Set,Ya=new Set;function Zn(t,e,r,a={}){function n(i){if(a.capture||Ka.call(e,i),!i.cancelBubble)return lr(()=>r?.call(this,i))}return t.startsWith("pointer")||t.startsWith("touch")||t==="wheel"?(n.__removed=!1,Nt(()=>{n.__removed||e.addEventListener(t,n,a)})):e.addEventListener(t,n,a),n}function xt(t,e,r,a,n){var i={capture:a,passive:n},l=Zn(t,e,r,i);(e===document.body||e===window||e===document||e instanceof HTMLMediaElement)&&ya(()=>{l.__removed=!0,e.removeEventListener(t,l,i)})}function y(t,e,r){(e[Dr]??={})[t]=r}function at(t){for(var e=0;e<t.length;e++)Xn.add(t[e]);for(var r of Ya)r(t)}let Na=null,za=!1;function Ka(t){var e=this,r=e.ownerDocument,a=t.type,n=t.composedPath?.()||[],i=n[0]||t.target;Na=t,za||(za=!0,setTimeout(()=>{za=!1,Na=null}));var l=0,c=Na===t&&t[Dr];if(c){var d=n.indexOf(c);if(d!==-1&&(e===document||e===window)){t[Dr]=e;return}var f=n.indexOf(e);if(f===-1)return;d<=f&&(l=d)}if(i=n[l]||t.target,i!==e){nn(t,"currentTarget",{configurable:!0,get(){return i||r}});var h=Me,p=ke;zt(null),Lt(null);try{for(var g,b=[];i!==null&&i!==e;){try{var w=i[Dr]?.[a];w!=null&&(!i.disabled||t.target===i)&&w.call(i,t)}catch(L){g?b.push(L):g=L}if(t.cancelBubble)break;l++,i=l<n.length?n[l]:null}if(g){for(let L of b)queueMicrotask(()=>{throw L});throw g}}finally{t[Dr]=e,delete t.currentTarget,zt(h),Lt(p)}}}const Do=globalThis?.window?.trustedTypes&&globalThis.window.trustedTypes.createPolicy("svelte-trusted-html",{createHTML:t=>t});function Ho(t){return Do?.createHTML(t)??t}function ei(t){var e=xa("template");return e.innerHTML=Ho(t.replaceAll("<!>","<!---->")),e.content}function or(t,e){var r=ke;r.nodes===null&&(r.nodes={start:t,end:e,a:null,t:null})}function S(t,e){var r=(e&Fi)!==0,a=(e&Di)!==0,n,i=!t.startsWith("<!>");return()=>{n===void 0&&(n=ei(i?t:"<!>"+t),r||(n=Rt(n)));var l=a||zn?document.importNode(n,!0):n.cloneNode(!0);if(r){var c=Rt(l),d=l.lastChild;or(c,d)}else or(l,l);return l}}function Vo(t,e,r="svg"){var a=!t.startsWith("<!>"),n=`<${r}>${a?t:"<!>"+t}</${r}>`,i;return()=>{if(!i){var l=ei(n),c=Rt(l);i=Rt(c)}var d=i.cloneNode(!0);return or(d,d),d}}function Qo(t,e){return Vo(t,e,"svg")}function wr(t=""){{var e=Ut(t+"");return or(e,e),e}}function fe(){var t=document.createDocumentFragment(),e=document.createComment(""),r=Ut();return t.append(e,r),or(e,r),t}function _(t,e){t!==null&&t.before(e)}function Uo(t){let e=0,r=ir(0),a;return()=>{ds()&&(s(r),vs(()=>(e===0&&(a=Zt(()=>t(()=>Ur(r)))),e+=1,()=>{Nt(()=>{e-=1,e===0&&(a?.(),a=void 0,Ur(r))})})))}}var Wo=Jt|Ir;function Bo(t,e,r,a){new Go(t,e,r,a)}class Go{parent;is_pending=!1;transform_error;#r;#l=null;#e;#i;#a;#t=null;#n=null;#s=null;#o=null;#f=0;#c=0;#h=!1;#g=new Map;#u=null;#p=Uo(()=>(this.#u=ir(this.#f),()=>{this.#u=null}));constructor(e,r,a,n){this.#r=e,this.#e=r,this.#i=i=>{var l=ke;l.b=this,l.f|=Ha,a(i)},this.parent=ke.b,this.transform_error=n??this.parent?.transform_error??(i=>i),this.#a=Lr(()=>{this.#x()},Wo)}#m(){try{this.#t=kt(()=>this.#i(this.#r))}catch(e){this.error(e)}}#w(e){const r=this.#e.failed,{reset:a,invoke_onerror:n}=this.#b(e);Nt(n),r&&(this.#s=kt(()=>{r(this.#r,()=>e,()=>a)}))}#b(e){var r=!1,a=!1;const n=()=>{if(!this.#d()){if(r){Wi();return}r=!0,a&&no(),this.#s!==null&&hr(this.#s,()=>{this.#s=null}),this.#y(()=>{this.#x()})}};return{reset:n,invoke_onerror:()=>{if(!this.#d())try{a=!0,this.#e.onerror?.(e,n),a=!1}catch(l){Dt(l,this.#a&&this.#a.parent)}}}}#d(){return(this.#a.f&(ht|Gr))!==0}#_(){const e=this.#e.pending;e&&(this.is_pending=!0,this.#n=kt(()=>e(this.#r)),Nt(()=>{if(!this.#d()){var r=this.#o=document.createDocumentFragment(),a=Ut(),n=!1;if(r.append(a),this.#t=this.#y(()=>{try{return kt(()=>this.#i(a))}catch(i){try{this.error(i),n=!0}catch(l){Dt(l,this.#a.parent)}return null}}),this.#t===null){this.#o=null,n&&this.#v(we);return}this.#c===0&&(this.#r.before(r),this.#o=null,hr(this.#n,()=>{this.#n=null}),this.#v(we))}}))}#x(){try{if(this.is_pending=this.has_pending_snippet(),this.#c=0,this.#f=0,this.#t=kt(()=>{this.#i(this.#r)}),this.#c>0){var e=this.#o=document.createDocumentFragment();hs(this.#t,e);const r=this.#e.pending;this.#n=kt(()=>r(this.#r))}else this.#v(we)}catch(r){this.error(r)}}#v(e){this.is_pending=!1,e.transfer_reactions(this.#g)}defer_effect(e){wn(e,this.#g)}is_rendered(){return!this.is_pending&&(!this.parent||this.parent.is_rendered())}has_pending_snippet(){return!!this.#e.pending}#y(e){var r=ke,a=Me,n=Ye;Lt(this.#a),zt(this.#a),Pr(this.#a.ctx);try{return nr.ensure(),e()}finally{Lt(r),zt(a),Pr(n)}}#k(e,r){if(!this.has_pending_snippet()){this.parent&&this.parent.#k(e,r);return}this.#c+=e,this.#c===0&&(this.#v(r),this.#n&&hr(this.#n,()=>{this.#n=null}),this.#o&&(this.#r.before(this.#o),this.#o=null))}update_pending_count(e,r){this.#k(e,r),this.#f+=e,!(!this.#u||this.#h)&&(this.#h=!0,Nt(()=>{this.#h=!1,this.#u&&Er(this.#u,this.#f)}))}get_effect_pending(){return this.#p(),s(this.#u)}error(e){if(e===pn||!this.#e.onerror&&!this.#e.failed)throw e;we?.is_fork?(this.#t&&we.skip_effect(this.#t),this.#n&&we.skip_effect(this.#n),this.#s&&we.skip_effect(this.#s),we.oncommit(()=>{this.#d()||this.#$(e)})):this.#$(e)}#$(e){this.#t&&(pt(this.#t),this.#t=null),this.#n&&(pt(this.#n),this.#n=null),this.#s&&(pt(this.#s),this.#s=null);let r=this.#e.failed;const a=n=>{if(this.#d())return;const{reset:i,invoke_onerror:l}=this.#b(n);l(),r&&!this.#d()&&(this.#s=this.#y(()=>{try{return kt(()=>{var c=ke;c.b=this,c.f|=Ha,r(this.#r,()=>n,()=>i)})}catch(c){return Dt(c,this.#a.parent),null}}))};Nt(()=>{if(!this.#d()){var n;try{n=this.transform_error(e)}catch(i){Dt(i,this.#a&&this.#a.parent);return}n!==null&&typeof n=="object"&&typeof n.then=="function"?n.then(a,i=>Dt(i,this.#a&&this.#a.parent)):a(n)}})}}let da=!0;function ea(t){da=t}function I(t,e){var r=e==null?"":typeof e=="object"?`${e}`:e;r!==(t[Ua]??=t.nodeValue)&&(t[Ua]=r,t.nodeValue=`${r}`)}function Yo(t,e){return Ko(t,e)}const ta=new Map;function Ko(t,{target:e,anchor:r,props:a={},events:n,context:i,intro:l=!0,transformError:c}){ko();var d=void 0,f=Ao(()=>{var h=r??e.appendChild(Ut());Bo(h,{pending:()=>{}},b=>{Je({});var w=Ye;i&&(w.c=i),n&&(a.$$events=n),da=l,d=t(b,a)||ns(),da=!0,Xe()},c);var p=new Set,g=b=>{for(var w=0;w<b.length;w++){var L=b[w];if(!p.has(L)){p.add(L);var k=Fo(L);for(const z of[e,document]){var M=ta.get(z);M===void 0&&(M=new Map,ta.set(z,M));var A=M.get(L);A===void 0?(z.addEventListener(L,Ka,{passive:k}),M.set(L,1)):M.set(L,A+1)}}}};return g(ba(Xn)),Ya.add(g),()=>{for(var b of p)for(const k of[e,document]){var w=ta.get(k),L=w.get(b);--L==0?(k.removeEventListener(b,Ka),w.delete(b),w.size===0&&ta.delete(k)):w.set(b,L)}Ya.delete(g),h!==r&&h.parentNode?.removeChild(h)}});return Jo.set(d,f),d}let Jo=new WeakMap;class ka{anchor;#r=new Map;#l=new Map;#e=new Map;#i=new Set;#a=!0;constructor(e,r=!0){this.anchor=e,this.#a=r}#t=e=>{if(this.#r.has(e)){var r=this.#r.get(e),a=this.#l.get(r);if(a)ua(a),this.#i.delete(r);else{var n=this.#e.get(r);n&&(ua(n.effect),this.#l.set(r,n.effect),this.#e.delete(r),n.fragment.lastChild.remove(),this.anchor.before(n.fragment),a=n.effect)}for(const[i,l]of this.#r){if(this.#r.delete(i),i===e)break;const c=this.#e.get(l);c&&(pt(c.effect),this.#e.delete(l))}for(const[i,l]of this.#l){if(i===r||this.#i.has(i))continue;const c=()=>{if(Array.from(this.#r.values()).includes(i)){var f=document.createDocumentFragment();hs(l,f),f.append(Ut()),this.#e.set(i,{effect:l,fragment:f})}else pt(l);this.#i.delete(i),this.#l.delete(i)};this.#a||!a?(this.#i.add(i),hr(l,c,!1)):c()}}};#n=e=>{this.#r.delete(e);const r=Array.from(this.#r.values());for(const[a,n]of this.#e)r.includes(a)||(pt(n.effect),this.#e.delete(a))};ensure(e,r){var a=we,n=Rn();if(r&&!this.#l.has(e)&&!this.#e.has(e))if(n){var i=document.createDocumentFragment(),l=Ut();i.append(l),this.#e.set(e,{effect:kt(()=>r(l)),fragment:i})}else this.#l.set(e,kt(()=>r(this.anchor)));if(this.#r.set(a,e),n){for(const[c,d]of this.#l)c===e?a.unskip_effect(d):a.skip_effect(d);for(const[c,d]of this.#e)c===e?a.unskip_effect(d.effect):a.skip_effect(d.effect);a.oncommit(this.#t),a.ondiscard(this.#n)}else this.#t(a)}}function F(t,e,r=!1){var a=new ka(t),n=r?Jt:0;function i(l,c){a.ensure(l,c)}Lr(()=>{var l=!1;e((c,d=0)=>{l=!0,i(d,c)}),l||i(-1,null)},n)}const Xo=Symbol("NaN");function ps(t,e,r){var a=new ka(t),n=!zr();Lr(()=>{var i=e();i!==i&&(i=Xo),n&&i!==null&&typeof i=="object"&&(i={}),a.ensure(i,r)})}function _r(t,e){return e}function Zo(t,e,r){for(var a=[],n=e.length,i,l=e.length,c=0;c<n;c++){let p=e[c];hr(p,()=>{if(i){if(i.pending.delete(p),i.done.add(p),i.pending.size===0){var g=t.outrogroups;Ja(t,ba(i.done)),g.delete(i),g.size===0&&(t.outrogroups=null)}}else l-=1},!1)}if(l===0){var d=a.length===0&&r!==null&&t.pending.size===0;if(d){var f=r,h=f.parentNode;$o(h),h.append(f),t.items.clear()}Ja(t,e,!d)}else i={pending:new Set(e),done:new Set},(t.outrogroups??=new Set).add(i)}function Ja(t,e,r=!0){var a;if(t.pending.size>0){a=new Set;for(const l of t.pending.values())for(const c of l)a.add(t.items.get(c).e)}for(var n=0;n<e.length;n++){var i=e[n];if(a?.has(i)){i.f|=Ht;const l=document.createDocumentFragment();hs(i,l)}else pt(e[n],r)}}var Ns;function Ve(t,e,r,a,n,i=null){var l=t,c=new Map,d=(e&fn)!==0;if(d){var f=t;l=f.appendChild(Ut())}var h=null,p=os(()=>{var A=r();return ga(A)?A:A==null?[]:ba(A)}),g=new Map,b=!0;function w(A){if((M.effect.f&ht)===0){M.pending.delete(A);var z=s(p);M.fallback=h,el(M,z,l,e,a),h!==null&&(z.length===0?(h.f&Ht)===0?ua(h):(h.f^=Ht,Hr(h,null,l)):hr(h,()=>{h=null}))}}function L(A){M.pending.delete(A)}var k=Lr(()=>{for(var A=s(p),z=A.length,q=new Set,C=we,$=Rn(),m=0;m<z;m+=1){var P=A[m],D=a(P,m),T=b?null:c.get(D);T?(T.v&&Er(T.v,P),T.i&&Er(T.i,m),$&&C.unskip_effect(T.e)):(T=tl(c,b?l:Ns??=Ut(),P,D,m,n,e,r),b||(T.e.f|=Ht),c.set(D,T)),q.add(D)}if(z===0&&i&&!h&&(b?h=kt(()=>i(l)):(h=kt(()=>i(Ns??=Ut())),h.f|=Ht)),z>q.size&&Ki(),!b)if(g.set(C,q),$){for(const[j,R]of c)q.has(j)||C.skip_effect(R.e);C.oncommit(w),C.ondiscard(L)}else w(C);s(p)}),M={effect:k,items:c,pending:g,outrogroups:null,fallback:h};b=!1}function qr(t){for(;t!==null&&(t.f&jt)===0;)t=t.next;return t}function el(t,e,r,a,n){var i=(a&Ii)!==0,l=e.length,c=t.items,d=qr(t.effect.first),f,h=null,p,g=[],b=[],w,L,k,M;if(i)for(M=0;M<l;M+=1)w=e[M],L=n(w,M),k=c.get(L).e,(k.f&Ht)===0&&(k.nodes?.a?.measure(),(p??=new Set).add(k));for(M=0;M<l;M+=1){if(w=e[M],L=n(w,M),k=c.get(L).e,t.outrogroups!==null)for(const T of t.outrogroups)T.pending.delete(k),T.done.delete(k);if((k.f&yt)!==0&&(ua(k),i&&(k.nodes?.a?.unfix(),(p??=new Set).delete(k))),(k.f&Ht)!==0)if(k.f^=Ht,k===d)Hr(k,null,r);else{var A=h?h.next:d;k===t.effect.last&&(t.effect.last=k.prev),k.prev&&(k.prev.next=k.next),k.next&&(k.next.prev=k.prev),er(t,h,k),er(t,k,A),Hr(k,A,r),h=k,g=[],b=[],d=qr(h.next);continue}if(k!==d){if(f!==void 0&&f.has(k)){if(g.length<b.length){var z=b[0],q;h=z.prev;var C=g[0],$=g[g.length-1];for(q=0;q<g.length;q+=1)Hr(g[q],z,r);for(q=0;q<b.length;q+=1)f.delete(b[q]);er(t,C.prev,$.next),er(t,h,C),er(t,$,z),d=z,h=$,M-=1,g=[],b=[]}else f.delete(k),Hr(k,d,r),er(t,k.prev,k.next),er(t,k,h===null?t.effect.first:h.next),er(t,h,k),h=k;continue}for(g=[],b=[];d!==null&&d!==k;)(f??=new Set).add(d),b.push(d),d=qr(d.next);if(d===null)continue}(k.f&Ht)===0&&g.push(k),h=k,d=qr(k.next)}if(t.outrogroups!==null){for(const T of t.outrogroups)T.pending.size===0&&(Ja(t,ba(T.done)),t.outrogroups?.delete(T));t.outrogroups.size===0&&(t.outrogroups=null)}if(d!==null||f!==void 0){var m=[];if(f!==void 0)for(k of f)(k.f&yt)===0&&m.push(k);for(;d!==null;)(d.f&yt)===0&&d!==t.fallback&&m.push(d),d=qr(d.next);var P=m.length;if(P>0){var D=(a&fn)!==0&&l===0?r:null;if(i){for(M=0;M<P;M+=1)m[M].nodes?.a?.measure();for(M=0;M<P;M+=1)m[M].nodes?.a?.fix()}Zo(t,m,D)}}i&&Nt(()=>{if(p!==void 0)for(k of p)k.nodes?.a?.apply()})}function tl(t,e,r,a,n,i,l,c){var d=(l&Ei)!==0?(l&Ni)===0?yo(r,!1,!1):ir(r):null,f=(l&Mi)!==0?ir(n):null;return{v:d,i:f,e:kt(()=>(i(e,d??r,f??n,c),()=>{t.delete(a)}))}}function Hr(t,e,r){if(t.nodes)for(var a=t.nodes.start,n=t.nodes.end,i=e&&(e.f&Ht)===0?e.nodes.start:r;a!==null;){var l=Xr(a);if(i.before(a),a===n)return;a=l}}function er(t,e,r){e===null?t.effect.first=r:e.next=r,r===null?t.effect.last=e:r.prev=e}function zs(t,e,r=!1,a=!1,n=!1,i=!1,l){var c=t,d="";if(r)var f=t;U(()=>{var h=ke;if(d!==(d=e()??"")){if(r&&!bn){h.nodes=null,f.innerHTML=d,d!==""&&or(Rt(f),f.lastChild);return}if(h.nodes!==null&&(Dn(h.nodes.start,h.nodes.end),h.nodes=null),d!==""){var p=a?gn:n?Hi:void 0,g=xa(a?"svg":n?"math":"template",p);g.innerHTML=d;var b=a||n?g:g.content;if(or(Rt(b),b.lastChild),a||n)for(;Rt(b);)c.before(Rt(b));else c.before(b)}}})}function $e(t,e,r,a,n,i){if(e.$$host?.$$shadowRoot){const d=xa("slot");_(t,d);return}var l=e.$$slots?.[r],c=!1;l===!0&&(l=e.children,c=!0),l===void 0||l(t,c?()=>a:a)}function rl(t,e,...r){var a=new ka(t);Lr(()=>{const n=e()??null;a.ensure(n,n&&(i=>n(i,...r)))},Jt)}const al=()=>performance.now(),Kt={tick:t=>requestAnimationFrame(t),now:()=>al(),tasks:new Set};function ti(){const t=Kt.now();Kt.tasks.forEach(e=>{e.c(t)||(Kt.tasks.delete(e),e.f())}),Kt.tasks.size!==0&&Kt.tick(ti)}function sl(t){let e;return Kt.tasks.size===0&&Kt.tick(ti),{promise:new Promise(r=>{Kt.tasks.add(e={c:t,f:r})}),abort(){Kt.tasks.delete(e)}}}function ra(t,e){lr(()=>{t.dispatchEvent(new CustomEvent(e))})}function nl(t){if(t==="float")return"cssFloat";if(t==="offset")return"cssOffset";if(t.startsWith("--"))return t;const e=t.split("-");return e.length===1?e[0]:e[0]+e.slice(1).map(r=>r[0].toUpperCase()+r.slice(1)).join("")}function Ls(t){const e={},r=t.split(";");for(const a of r){const[n,i]=a.split(":");if(!n||i===void 0)break;const l=nl(n.trim());e[l]=i.trim()}return e}const il=t=>t;function gt(t,e,r,a){var n=(t&qi)!==0,i=(t&Oi)!==0,l=n&&i,c=(t&ji)!==0,d=l?"both":n?"in":"out",f,h=e.inert,p=e.style.overflow,g,b;function w(){return lr(()=>f??=r()(e,a?.()??{},{direction:d}))}var L={is_global:c,in(){if(e.inert=h,!n){b?.abort(),b?.reset?.();return}i||g?.abort(),g=Xa(e,w(),b,1,()=>{ra(e,"introstart")},()=>{ra(e,"introend"),g?.abort(),g=f=void 0,e.style.overflow=p})},out(z){if(!i){z?.(),f=void 0;return}e.inert=!0,b=Xa(e,w(),g,0,()=>{ra(e,"outrostart")},()=>{ra(e,"outroend"),z?.()})},stop:()=>{g?.abort(),b?.abort()}},k=ke;if((k.nodes.t??=[]).push(L),n&&da){var M=c;if(!M){for(var A=k.parent;A&&(A.f&Jt)!==0;)for(;(A=A.parent)&&(A.f&It)===0;);M=!A||(A.f&gr)!==0}M&&wa(()=>{Zt(()=>L.in())})}}function Xa(t,e,r,a,n,i){var l=a===1,c=!1;if(xr(e)){var d;return Nt(()=>{if(!c){var L=e({direction:l?"in":"out"});d=Xa(t,L,r,a,n,i)}}),{abort:()=>{c=!0,d?.abort()},deactivate:()=>d?.deactivate(),reset:()=>d?.reset(),t:()=>d?.t()??1-a}}if(r?.deactivate(),!e?.duration&&!e?.delay)return n(),i(),{abort:yr,deactivate:yr,reset:yr,t:()=>a};const{delay:f=0,css:h,tick:p,easing:g=il}=e;var b,w=()=>1-a;return Nt(()=>{if(!c){var L=[];if(l&&r===void 0&&(p&&p(0,1),h)){var k=Ls(h(0,1));L.push(k,k)}b=t.animate(L,{duration:f,fill:"forwards"}),b.onfinish=()=>{b.cancel(),n();var M=r?.t()??1-a;r?.abort();var A=a-M,z=e.duration*Math.abs(A),q=[];if(z>0){var C=!1;if(h)for(var $=Math.ceil(z/16.666666666666668),m=0;m<=$;m+=1){var P=M+A*g(m/$),D=Ls(h(P,1-P));q.push(D),C||=D.overflow==="hidden"}C&&(t.style.overflow="hidden"),w=()=>{var T=b.currentTime;return M+A*g(T/z)},p&&sl(()=>{if(b.playState!=="running")return!1;var T=w();return p(T,1-T),!0})}b=t.animate(q,{duration:z,fill:"forwards"}),b.onfinish=()=>{w=()=>a,p?.(a,1-a),i()}}}}),{abort:()=>{c=!0,b&&(b.cancel(),b.effect=null,b.onfinish=yr)},deactivate:()=>{i=yr},reset:()=>{a===0&&p?.(1,0)},t:()=>w()}}function ol(t,e,r,a,n,i){var l=null,c=t,d=new ka(c,!1);Lr(()=>{const f=e()||null;var h=gn;if(f===null){d.ensure(null,null),ea(!0);return}return d.ensure(f,p=>{if(f){var g=bn;if(l=xa(f,h),or(l,l),a){var b=null,w=l.appendChild(Ut());a(l,w),b?.remove()}ke.nodes.end=l,g||p.before(l)}}),ea(!0),()=>{f&&ea(!1)}},Jt),ya(()=>{ea(!0)})}function ll(t,e){var r=void 0,a;jn(()=>{r!==(r=e())&&(a&&(pt(a),a=null),r&&(a=kt(()=>{wa(()=>r(t))})))})}function ri(t){var e,r,a="";if(typeof t=="string"||typeof t=="number")a+=t;else if(typeof t=="object")if(Array.isArray(t)){var n=t.length;for(e=0;e<n;e++)t[e]&&(r=ri(t[e]))&&(a&&(a+=" "),a+=r)}else for(r in t)t[r]&&(a&&(a+=" "),a+=r);return a}function cl(){for(var t,e,r=0,a="",n=arguments.length;r<n;r++)(t=arguments[r])&&(e=ri(t))&&(a&&(a+=" "),a+=e);return a}function ul(t){return typeof t=="object"?cl(t):t??""}const Cs=[...` 	
\r\f \v\uFEFF`];function dl(t,e,r){var a=t==null?"":""+t;if(e&&(a=a?a+" "+e:e),r){for(var n of Object.keys(r))if(r[n])a=a?a+" "+n:n;else if(a.length)for(var i=n.length,l=0;(l=a.indexOf(n,l))>=0;){var c=l+i;(l===0||Cs.includes(a[l-1]))&&(c===a.length||Cs.includes(a[c]))?a=(l===0?"":a.substring(0,l))+a.substring(c+1):l=c}}return a===""?null:a}function Rs(t,e=!1){var r=e?" !important;":";",a="";for(var n of Object.keys(t)){var i=t[n];i!=null&&i!==""&&(a+=" "+n+": "+i+r)}return a}function La(t){return t[0]!=="-"||t[1]!=="-"?t.toLowerCase():t}function vl(t,e){if(e){var r="",a,n;if(Array.isArray(e)?(a=e[0],n=e[1]):a=e,t){t=String(t).replaceAll(/\/\*.*?\*\//g,"").trim();var i=!1,l=0,c=!1,d=[];a&&d.push(...Object.keys(a).map(La)),n&&d.push(...Object.keys(n).map(La));var f=0,h=-1;const L=t.length;for(var p=0;p<L;p++){var g=t[p];if(c?g==="/"&&t[p-1]==="*"&&(c=!1):i?i===g&&(i=!1):g==="/"&&t[p+1]==="*"?c=!0:g==='"'||g==="'"?i=g:g==="("?l++:g===")"&&l--,!c&&i===!1&&l===0){if(g===":"&&h===-1)h=p;else if(g===";"||p===L-1){if(h!==-1){var b=La(t.substring(f,h).trim());if(!d.includes(b)){g!==";"&&p++;var w=t.substring(f,p).trim();r+=" "+w+";"}}f=p+1,h=-1}}}}return a&&(r+=Rs(a)),n&&(r+=Rs(n,!0)),r=r.trim(),r===""?null:r}return t==null?null:String(t)}function X(t,e,r,a,n,i){var l=t[Va];if(l!==r||l===void 0){var c=dl(r,a,i);c==null?t.removeAttribute("class"):e?t.className=c:t.setAttribute("class",c),t[Va]=r}else if(i&&n!==i)for(var d in i){var f=!!i[d];(n==null||f!==!!n[d])&&t.classList.toggle(d,f)}return i}function Ca(t,e={},r,a){for(var n in r){var i=r[n];e[n]!==i&&(r[n]==null?t.style.removeProperty(n):t.style.setProperty(n,i,a))}}function lt(t,e,r,a){var n=t[Qa];if(n!==e){var i=vl(e,a);i==null?t.removeAttribute("style"):t.style.cssText=i,t[Qa]=e}else a&&(Array.isArray(a)?(Ca(t,r?.[0],a[0]),Ca(t,r?.[1],a[1],"important")):Ca(t,r,a));return a}function ai(t,e){e?t.hasAttribute("selected")||t.setAttribute("selected",""):t.removeAttribute("selected")}function qs(t,e){var r=!("__defaultValue"in t);!r&&t.__defaultValue===e||(t.__defaultValue=e,si(t,!r||"__value"in t))}function si(t,e){var r=t.__defaultValue,a=t.multiple,n=a?r??[]:null;if(!(a&&!ga(n))){var i=t.selectedIndex,l=e&&a?new Set(t.selectedOptions):null;for(var c of t.options){var d=Za(c);ai(c,a?n.includes(d):Nn(d,r))}if(e)if(l!==null)for(c of t.options){var f=l.has(c);c.selected!==f&&(c.selected=f)}else t.selectedIndex!==i&&(t.selectedIndex=i)}}function va(t,e,r=!1){if(t.multiple){if(e==null)return;if(!ga(e))return Ui();for(var a of t.options)a.selected=e.includes(Za(a));return}for(a of t.options){var n=Za(a);if(Nn(n,e)){a.selected=!0;return}}(!r||e!==void 0)&&(t.selectedIndex=-1)}function ni(t){var e=new MutationObserver(r=>{r.every(fl)||("__defaultValue"in t&&si(t,!1),"__value"in t&&va(t,t.__value))});e.observe(t,{childList:!0,subtree:!0,attributes:!0,attributeFilter:["value"]}),ya(()=>{e.disconnect()})}function Za(t){return"__value"in t?t.__value:t.value}function fl(t){if(t.target.closest("selectedcontent")!==null)return!0;if(t.type==="childList"){var e=[...t.addedNodes,...t.removedNodes];return e.length>0&&e.every(r=>r.nodeName==="SELECTEDCONTENT")}return!1}const Or=Symbol("class"),jr=Symbol("style"),ii=Symbol("is custom element"),oi=Symbol("is html"),hl=ma?"input":"INPUT",pl=ma?"option":"OPTION",li=ma?"select":"SELECT",ci=ma?"progress":"PROGRESS";function es(t,e){var r=$a(t);r.value===(r.value=e??void 0)||t.value===e&&(e!==0||t.nodeName!==ci)||(t.value=e??"")}function Wr(t,e){var r=$a(t);r.checked!==(r.checked=e??void 0)&&(t.checked=e)}function it(t,e,r,a){var n=$a(t),i=n[e];i!==(n[e]=r)&&(e==="loading"&&(t[Ai]=r),r==null?t.removeAttribute(e):typeof r!="string"&&ui(t).has(e)?t[e]=r:t.setAttribute(e,r))}function _l(t,e,r,a,n=!1,i=!1){var l=$a(t),c=l[ii],d=!l[oi],f=e||{},h=t.nodeName===pl,p=t.nodeName===li;try{for(var g in e)!(g in r)&&g[0]+g[1]!=="$$"&&(r[g]=null);r.class?r.class=ul(r.class):(a||r[Or])&&(r.class=null),r[jr]&&(r.style??=null);var b=ui(t);if(t.nodeName===hl&&"type"in r&&("value"in r||"__value"in r)){var w=r.type;(w!==f.type||w===void 0&&t.hasAttribute("type"))&&(f.type=w,it(t,"type",w,i))}for(const $ in r){let m=r[$];if(h&&$==="value"&&m==null){t.value=t.__value="",f[$]=m;continue}if($==="class"){var L=t.namespaceURI==="http://www.w3.org/1999/xhtml";X(t,L,m,a,e?.[Or],r[Or]),f[$]=m,f[Or]=r[Or];continue}if($==="style"){lt(t,m,e?.[jr],r[jr]),f[$]=m,f[jr]=r[jr];continue}var k=f[$];if(!(m===k&&!(m===void 0&&t.hasAttribute($)))){f[$]=m;var M=$[0]+$[1];if(M!=="$$")if(M==="on"){const P={},D="$$"+$;let T=$.slice(2);var A=Ro(T);if(Lo(T)&&(T=T.slice(0,-7),P.capture=!0),!A&&k){if(m!=null)continue;t.removeEventListener(T,f[D],P),f[D]=null}if(A)y(T,t,m),at([T]);else if(m!=null){let j=function(R){f[$].call(this,R)};var C=j;f[D]=Zn(T,t,j,P)}}else if($==="style")it(t,$,m);else if($==="autofocus")uo(t,!!m);else if(!c&&($==="__value"||$==="value"&&m!=null))t.__value=m,(k==null||t.value!==m||m===0&&t.nodeName===ci)&&(t.value=m);else if($==="selected"&&h)ai(t,m);else{var z=$;d||(z=Oo(z));var q=z==="defaultValue"||z==="defaultChecked";if(p&&z==="defaultValue")continue;if(m==null&&!c&&!q)if(l[$]=null,z==="value"||z==="checked"){let P=t;const D=e===void 0;if(z==="value"){let T=P.defaultValue;P.removeAttribute(z),P.defaultValue=T,P.value=P.__value=D?T:null}else{let T=P.defaultChecked;P.removeAttribute(z),P.defaultChecked=T,P.checked=D?T:!1}}else t.removeAttribute($);else q||(c||typeof m!="string")&&b.has(z)?(t[z]=m,z in l&&(l[z]=nt)):typeof m!="function"&&it(t,z,m,i)}}}}finally{}return f}function Os(t,e,r=[],a=[],n=[],i,l=!1,c=!1){kn(n,r,a,d=>{var f=void 0,h={},p=t.nodeName===li,g=!1;if(jn(()=>{var w=e(...d.map(s)),L=_l(t,f,w,i,l,c);if(g&&p){var k=t;"defaultValue"in w&&qs(k,w.defaultValue),"value"in w&&va(k,w.value)}for(let A of Object.getOwnPropertySymbols(h))w[A]||pt(h[A]);for(let A of Object.getOwnPropertySymbols(w)){var M=w[A];A.description===Vi&&(!f||M!==f[A])&&(h[A]&&pt(h[A]),h[A]=kt(()=>ll(t,()=>M))),L[A]=M}f=L}),p){var b=t;wa(()=>{var w=f;"defaultValue"in w&&qs(b,w.defaultValue),va(b,w.value,!0),ni(b)})}g=!0})}function $a(t){return t[vn]??={[ii]:t.nodeName.includes("-"),[oi]:t.namespaceURI===_n}}var js=new Map;function ui(t){var e=t.getAttribute("is")||t.nodeName,r=js.get(e);if(r)return r;js.set(e,r=new Set);for(var a,n=t,i=Element.prototype;i!==n;){a=on(n);for(var l in a)a[l].set&&l!=="innerHTML"&&l!=="textContent"&&l!=="innerText"&&r.add(l);n=rs(n)}return r}function fa(t,e,r=e){var a=new WeakSet;fo(t,"input",async n=>{var i=n?t.defaultValue:t.value;if(i=Ra(t)?qa(i):i,r(i),we!==null&&a.add(we),await zo(),i!==(i=e())){var l=t.selectionStart,c=t.selectionEnd,d=t.value.length;if(t.value=i??"",c!==null){var f=t.value.length;l===c&&c===d&&f>d?(t.selectionStart=f,t.selectionEnd=f):(t.selectionStart=l,t.selectionEnd=Math.min(c,f))}}}),Zt(e)==null&&t.value&&(r(Ra(t)?qa(t.value):t.value),we!==null&&a.add(we)),vs(()=>{var n=e();if(t===document.activeElement){var i=we;if(a.has(i))return}Ra(t)&&n===qa(t.value)||t.type==="date"&&!n&&!t.value||n!==t.value&&(t.value=n??"")})}function Ra(t){var e=t.type;return e==="number"||e==="range"}function qa(t){return t===""?null:+t}function Oa(t,e){return t===e||t?.[Vt]===e}function br(t=ns(),e,r,a){var n=Ye.r,i=ke;return wa(()=>{var l,c;return vs(()=>{l=c,c=a?.()||[],Zt(()=>{Oa(r(...c),t)||(e(t,...c),l&&Oa(r(...l),t)&&e(null,...l))})}),()=>{let d=i;for(;d!==n&&d.parent!==null&&d.parent.f&Gr;)d=d.parent;const f=()=>{c&&Oa(r(...c),t)&&e(null,...c)},h=d.teardown;d.teardown=()=>{f(),h?.()}}}),t}function di(t=!1){const e=Ye,r=e.l.u;if(!r)return;let a=()=>ur(e.s);if(t){let n=0,i={};const l=Ar(()=>{let c=!1;const d=e.s;for(const f in d)d[f]!==i[f]&&(i[f]=d[f],c=!0);return c&&n++,n});a=()=>s(l)}r.b.length&&Po(()=>{Fs(e,a),Da(r.b)}),At(()=>{const n=Zt(()=>r.m.map(Si));return()=>{for(const i of n)typeof i=="function"&&i()}}),r.a.length&&At(()=>{Fs(e,a),Da(r.a)})}function Fs(t,e){if(t.l.s)for(const r of t.l.s)s(r);e()}let aa=!1;function gl(t){var e=aa;try{return aa=!1,[t(),aa]}finally{aa=e}}const bl={get(t,e){if(!t.exclude.includes(e))return s(t.version),e in t.special?t.special[e]():t.props[e]},set(t,e,r){if(!(e in t.special)){var a=ke;try{Lt(t.parent_effect),t.special[e]=ft({get[e](){return t.props[e]}},e,hn)}finally{Lt(a)}}return t.special[e](r),Ts(t.version),!0},getOwnPropertyDescriptor(t,e){if(!t.exclude.includes(e)&&e in t.props)return{enumerable:!0,configurable:!0,value:t.props[e]}},deleteProperty(t,e){return t.exclude.includes(e)||(t.exclude.push(e),Ts(t.version)),!0},has(t,e){return t.exclude.includes(e)?!1:e in t.props},ownKeys(t){return Reflect.ownKeys(t.props).filter(e=>!t.exclude.includes(e))}};function xe(t,e){return new Proxy({props:t,exclude:e,special:{},version:ir(0),parent_effect:ke},bl)}const ml={get(t,e){let r=t.props.length;for(;r--;){let a=t.props[r];if(xr(a)&&(a=a()),typeof a=="object"&&a!==null&&e in a)return a[e]}},set(t,e,r){let a=t.props.length;for(;a--;){let n=t.props[a];xr(n)&&(n=n());const i=rr(n,e);if(i&&i.set)return i.set(r),!0}return!1},getOwnPropertyDescriptor(t,e){let r=t.props.length;for(;r--;){let a=t.props[r];if(xr(a)&&(a=a()),typeof a=="object"&&a!==null&&e in a){const n=rr(a,e);return n&&!n.configurable&&(n.configurable=!0),n}}},has(t,e){if(e===Vt||e===dn)return!1;for(let r of t.props)if(xr(r)&&(r=r()),r!=null&&e in r)return!0;return!1},ownKeys(t){const e=[];for(let r of t.props)if(xr(r)&&(r=r()),!!r){for(const a in r)e.includes(a)||e.push(a);for(const a of Object.getOwnPropertySymbols(r))e.includes(a)||e.push(a)}return e}};function Pe(...t){return new Proxy({props:t},ml)}function ft(t,e,r,a){var n=!Nr||(r&Li)!==0,i=(r&Ci)!==0,l=(r&Ri)!==0,c=a,d=!0,f=void 0,h=()=>l&&n?(f??=Ar(a),s(f)):(d&&(d=!1,c=l?Zt(a):a),c);let p;if(i){var g=Vt in t||dn in t;p=rr(t,e)?.set??(g&&e in t?q=>t[e]=q:void 0)}var b,w=!1;i?[b,w]=gl(()=>t[e]):b=t[e],b===void 0&&a!==void 0&&(b=h(),p&&(n&&to(),p(b)));var L;if(n?L=()=>{var q=t[e];return q===void 0?h():(d=!0,q)}:L=()=>{var q=t[e];return q!==void 0&&(c=void 0),q===void 0?c:q},n&&(r&hn)===0)return L;if(p){var k=t.$$legacy;return(function(q,C){return arguments.length>0?((!n||!C||k||w)&&p(C?L():q),q):L()})}var M=!1,A=((r&zi)!==0?Ar:os)(()=>(M=!1,L()));i&&s(A);var z=ke;return(function(q,C){if(arguments.length>0){const $=C?s(A):n&&i?Ke(q):q;return x(A,$),M=!0,c!==void 0&&(c=$),q}return Xt&&M||(z.f&ht)!==0?A.v:s(A)})}function Sa(t){Ye===null&&Gi(),Nr&&Ye.l!==null?xl(Ye).m.push(t):At(()=>{const e=Zt(t);if(typeof e=="function")return e})}function xl(t){var e=t.l;return e.u??={a:[],b:[],m:[]}}const yl="5";typeof window<"u"&&((window.__svelte??={}).v??=new Set).add(yl);oo();/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 * 
 * Copyright (c) 2026 Lucide Icons and Contributors
 * 
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 * 
 * ---
 * 
 * The following Lucide icons are derived from the Feather project:
 * 
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 * 
 * The MIT License (MIT) (for the icons listed above)
 * 
 * Copyright (c) 2013-present Cole Bemis
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 * 
 */const wl={xmlns:"http://www.w3.org/2000/svg",width:24,height:24,viewBox:"0 0 24 24",fill:"none",stroke:"currentColor","stroke-width":2,"stroke-linecap":"round","stroke-linejoin":"round"};/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 * 
 * Copyright (c) 2026 Lucide Icons and Contributors
 * 
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 * 
 * ---
 * 
 * The following Lucide icons are derived from the Feather project:
 * 
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 * 
 * The MIT License (MIT) (for the icons listed above)
 * 
 * Copyright (c) 2013-present Cole Bemis
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 * 
 */const kl=t=>{for(const e in t)if(e.startsWith("aria-")||e==="role"||e==="title")return!0;return!1};/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 * 
 * Copyright (c) 2026 Lucide Icons and Contributors
 * 
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 * 
 * ---
 * 
 * The following Lucide icons are derived from the Feather project:
 * 
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 * 
 * The MIT License (MIT) (for the icons listed above)
 * 
 * Copyright (c) 2013-present Cole Bemis
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 * 
 */const Ds=(...t)=>t.filter((e,r,a)=>!!e&&e.trim()!==""&&a.indexOf(e)===r).join(" ").trim();var $l=Qo("<svg><!><!></svg>");function Ae(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]),a=xe(r,["name","color","size","strokeWidth","absoluteStrokeWidth","iconNode"]);Je(e,!1);let n=ft(e,"name",8,void 0),i=ft(e,"color",8,"currentColor"),l=ft(e,"size",8,24),c=ft(e,"strokeWidth",8,2),d=ft(e,"absoluteStrokeWidth",8,!1),f=ft(e,"iconNode",24,()=>[]);di();var h=$l();Os(h,(b,w,L)=>({...wl,...b,...a,width:l(),height:l(),stroke:i(),"stroke-width":w,class:L}),[()=>kl(a)?void 0:{"aria-hidden":"true"},()=>(ur(d()),ur(c()),ur(l()),Zt(()=>d()?Number(c())*24/Number(l()):c())),()=>(ur(Ds),ur(n()),ur(r),Zt(()=>Ds("lucide-icon","lucide",n()?`lucide-${n()}`:"",r.class)))]);var p=v(h);Ve(p,1,f,_r,(b,w)=>{var L=Z(()=>Ti(s(w),2));let k=()=>s(L)[0],M=()=>s(L)[1];var A=fe(),z=se(A);ol(z,k,!0,(q,C)=>{Os(q,()=>({...M()}))}),_(b,A)});var g=u(p);$e(g,e,"default",{}),_(t,h),Xe()}function Sl(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M5 12h14"}],["path",{d:"m12 5 7 7-7 7"}]];Ae(t,Pe({name:"arrow-right"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Tl(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M19 17h2c.6 0 1-.4 1-1v-3c0-.9-.7-1.7-1.5-1.9C18.7 10.6 16 10 16 10s-1.3-1.4-2.2-2.3c-.5-.4-1.1-.7-1.8-.7H5c-.6 0-1.1.4-1.4.9l-1.4 2.9A3.7 3.7 0 0 0 2 12v4c0 .6.4 1 1 1h2"}],["circle",{cx:"7",cy:"17",r:"2"}],["path",{d:"M9 17h6"}],["circle",{cx:"17",cy:"17",r:"2"}]];Ae(t,Pe({name:"car"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function ts(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"m15 18-6-6 6-6"}]];Ae(t,Pe({name:"chevron-left"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Hs(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"m9 18 6-6-6-6"}]];Ae(t,Pe({name:"chevron-right"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function vi(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["circle",{cx:"12",cy:"12",r:"10"}],["path",{d:"m16.24 7.76-1.804 5.411a2 2 0 0 1-1.265 1.265L7.76 16.24l1.804-5.411a2 2 0 0 1 1.265-1.265z"}]];Ae(t,Pe({name:"compass"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Pl(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["circle",{cx:"12",cy:"12",r:"10"}],["path",{d:"M6 12c0-1.7.7-3.2 1.8-4.2"}],["circle",{cx:"12",cy:"12",r:"2"}],["path",{d:"M18 12c0 1.7-.7 3.2-1.8 4.2"}]];Ae(t,Pe({name:"disc-3"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Al(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["circle",{cx:"9",cy:"12",r:"1"}],["circle",{cx:"9",cy:"5",r:"1"}],["circle",{cx:"9",cy:"19",r:"1"}],["circle",{cx:"15",cy:"12",r:"1"}],["circle",{cx:"15",cy:"5",r:"1"}],["circle",{cx:"15",cy:"19",r:"1"}]];Ae(t,Pe({name:"grip-vertical"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function El(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M3 14h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-7a9 9 0 0 1 18 0v7a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3"}]];Ae(t,Pe({name:"headphones"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Cr(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M2 9.5a5.5 5.5 0 0 1 9.591-3.676.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5l-5.492 5.313a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5"}]];Ae(t,Pe({name:"heart"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Ml(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8"}],["path",{d:"M3 10a2 2 0 0 1 .709-1.528l7-6a2 2 0 0 1 2.582 0l7 6A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"}]];Ae(t,Pe({name:"house"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Il(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"m16 6 4 14"}],["path",{d:"M12 6v14"}],["path",{d:"M8 8v12"}],["path",{d:"M4 4v16"}]];Ae(t,Pe({name:"library"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function fi(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M16 5H3"}],["path",{d:"M11 12H3"}],["path",{d:"M11 19H3"}],["path",{d:"M21 16V5"}],["circle",{cx:"18",cy:"16",r:"3"}]];Ae(t,Pe({name:"list-music"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function hi(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["line",{x1:"2",x2:"5",y1:"12",y2:"12"}],["line",{x1:"19",x2:"22",y1:"12",y2:"12"}],["line",{x1:"12",x2:"12",y1:"2",y2:"5"}],["line",{x1:"12",x2:"12",y1:"19",y2:"22"}],["circle",{cx:"12",cy:"12",r:"7"}],["circle",{cx:"12",cy:"12",r:"3"}]];Ae(t,Pe({name:"locate-fixed"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Nl(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M14 14a2 2 0 0 0 2-2V8h-2"}],["path",{d:"M22 17a2 2 0 0 1-2 2H6.828a2 2 0 0 0-1.414.586l-2.202 2.202A.71.71 0 0 1 2 21.286V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z"}],["path",{d:"M8 14a2 2 0 0 0 2-2V8H8"}]];Ae(t,Pe({name:"message-square-quote"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function zl(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M20.985 12.486a9 9 0 1 1-9.473-9.472c.405-.022.617.46.402.803a6 6 0 0 0 8.268 8.268c.344-.215.825-.004.803.401"}]];Ae(t,Pe({name:"moon"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Ta(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M9 18V5l12-2v13"}],["circle",{cx:"6",cy:"18",r:"3"}],["circle",{cx:"18",cy:"16",r:"3"}]];Ae(t,Pe({name:"music"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Ll(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["rect",{width:"18",height:"18",x:"3",y:"3",rx:"2"}],["path",{d:"M9 3v18"}]];Ae(t,Pe({name:"panel-left"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function _s(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M5.8 11.3 2 22l10.7-3.79"}],["path",{d:"M4 3h.01"}],["path",{d:"M22 8h.01"}],["path",{d:"M15 2h.01"}],["path",{d:"M22 20h.01"}],["path",{d:"m22 2-2.24.75a2.9 2.9 0 0 0-1.96 3.12c.1.86-.57 1.63-1.45 1.63h-.38c-.86 0-1.6.6-1.76 1.44L14 10"}],["path",{d:"m22 13-.82-.33c-.86-.34-1.82.2-1.98 1.11c-.11.7-.72 1.22-1.43 1.22H17"}],["path",{d:"m11 2 .33.82c.34.86-.2 1.82-1.11 1.98C9.52 4.9 9 5.52 9 6.23V7"}],["path",{d:"M11 13c1.93 1.93 2.83 4.17 2 5-.83.83-3.07-.07-5-2-1.93-1.93-2.83-4.17-2-5 .83-.83 3.07.07 5 2Z"}]];Ae(t,Pe({name:"party-popper"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function gs(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["rect",{x:"14",y:"3",width:"5",height:"18",rx:"1"}],["rect",{x:"5",y:"3",width:"5",height:"18",rx:"1"}]];Ae(t,Pe({name:"pause"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Rr(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M5 5a2 2 0 0 1 3.008-1.728l11.997 6.998a2 2 0 0 1 .003 3.458l-12 7A2 2 0 0 1 5 19z"}]];Ae(t,Pe({name:"play"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function pi(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M5 12h14"}],["path",{d:"M12 5v14"}]];Ae(t,Pe({name:"plus"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Cl(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M16.247 7.761a6 6 0 0 1 0 8.478"}],["path",{d:"M19.075 4.933a10 10 0 0 1 0 14.134"}],["path",{d:"M4.925 19.067a10 10 0 0 1 0-14.134"}],["path",{d:"M7.753 16.239a6 6 0 0 1 0-8.478"}],["circle",{cx:"12",cy:"12",r:"2"}]];Ae(t,Pe({name:"radio"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Rl(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"}],["path",{d:"M21 3v5h-5"}],["path",{d:"M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"}],["path",{d:"M8 16H3v5"}]];Ae(t,Pe({name:"refresh-cw"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function _i(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"m17 2 4 4-4 4"}],["path",{d:"M3 11v-1a4 4 0 0 1 4-4h14"}],["path",{d:"m7 22-4-4 4-4"}],["path",{d:"M21 13v1a4 4 0 0 1-4 4H3"}]];Ae(t,Pe({name:"repeat"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Vs(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"m21 21-4.34-4.34"}],["circle",{cx:"11",cy:"11",r:"8"}]];Ae(t,Pe({name:"search"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function ql(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M9.671 4.136a2.34 2.34 0 0 1 4.659 0 2.34 2.34 0 0 0 3.319 1.915 2.34 2.34 0 0 1 2.33 4.033 2.34 2.34 0 0 0 0 3.831 2.34 2.34 0 0 1-2.33 4.033 2.34 2.34 0 0 0-3.319 1.915 2.34 2.34 0 0 1-4.659 0 2.34 2.34 0 0 0-3.32-1.915 2.34 2.34 0 0 1-2.33-4.033 2.34 2.34 0 0 0 0-3.831A2.34 2.34 0 0 1 6.35 6.051a2.34 2.34 0 0 0 3.319-1.915"}],["circle",{cx:"12",cy:"12",r:"3"}]];Ae(t,Pe({name:"settings"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function gi(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"m18 14 4 4-4 4"}],["path",{d:"m18 2 4 4-4 4"}],["path",{d:"M2 18h1.973a4 4 0 0 0 3.3-1.7l5.454-8.6a4 4 0 0 1 3.3-1.7H22"}],["path",{d:"M2 6h1.972a4 4 0 0 1 3.6 2.2"}],["path",{d:"M22 18h-6.041a4 4 0 0 1-3.3-1.8l-.359-.45"}]];Ae(t,Pe({name:"shuffle"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function bs(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M17.971 4.285A2 2 0 0 1 21 6v12a2 2 0 0 1-3.029 1.715l-9.997-5.998a2 2 0 0 1-.003-3.432z"}],["path",{d:"M3 20V4"}]];Ae(t,Pe({name:"skip-back"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function ms(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M21 4v16"}],["path",{d:"M6.029 4.285A2 2 0 0 0 3 6v12a2 2 0 0 0 3.029 1.715l9.997-5.998a2 2 0 0 0 .003-3.432z"}]];Ae(t,Pe({name:"skip-forward"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Ol(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["circle",{cx:"12",cy:"12",r:"10"}],["circle",{cx:"12",cy:"12",r:"6"}],["circle",{cx:"12",cy:"12",r:"2"}]];Ae(t,Pe({name:"target"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function jl(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["line",{x1:"10",x2:"14",y1:"2",y2:"2"}],["line",{x1:"12",x2:"15",y1:"14",y2:"11"}],["circle",{cx:"12",cy:"14",r:"8"}]];Ae(t,Pe({name:"timer"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Fl(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M16 7h6v6"}],["path",{d:"m22 7-8.5 8.5-5-5L2 17"}]];Ae(t,Pe({name:"trending-up"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Dl(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"}],["path",{d:"M16 3.128a4 4 0 0 1 0 7.744"}],["path",{d:"M22 21v-2a4 4 0 0 0-3-3.87"}],["circle",{cx:"9",cy:"7",r:"4"}]];Ae(t,Pe({name:"users"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Qs(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M11 4.702a.705.705 0 0 0-1.203-.498L6.413 7.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.705.705 0 0 0 11 19.298z"}],["path",{d:"M16 9a5 5 0 0 1 0 6"}]];Ae(t,Pe({name:"volume-1"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function ha(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M11 4.702a.705.705 0 0 0-1.203-.498L6.413 7.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.705.705 0 0 0 11 19.298z"}],["path",{d:"M16 9a5 5 0 0 1 0 6"}],["path",{d:"M19.364 18.364a9 9 0 0 0 0-12.728"}]];Ae(t,Pe({name:"volume-2"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Us(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M11 4.702a.705.705 0 0 0-1.203-.498L6.413 7.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.705.705 0 0 0 11 19.298z"}],["line",{x1:"22",x2:"16",y1:"9",y2:"15"}],["line",{x1:"16",x2:"22",y1:"9",y2:"15"}]];Ae(t,Pe({name:"volume-x"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function xs(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M18 6 6 18"}],["path",{d:"m6 6 12 12"}]];Ae(t,Pe({name:"x"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Hl(t,e){const r=xe(e,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const a=[["path",{d:"M4 14a1 1 0 0 1-.78-1.63l9.9-10.2a.5.5 0 0 1 .86.46l-1.92 6.02A1 1 0 0 0 13 10h7a1 1 0 0 1 .78 1.63l-9.9 10.2a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14z"}]];Ae(t,Pe({name:"zap"},()=>r,{get iconNode(){return a},children:(n,i)=>{var l=fe(),c=se(l);$e(c,e,"default",{}),_(n,l)},$$slots:{default:!0}}))}function Mt(t){const e=t-1;return e*e*e+1}const Vl=t=>t;function bi(t){const e=t-1;return e*e*e+1}function Ws(t){const e=typeof t=="string"&&t.match(/^\s*(-?[\d.]+)([^\s]*)\s*$/);return e?[parseFloat(e[1]),e[2]||"px"]:[t,"px"]}function Pa(t,{delay:e=0,duration:r=400,easing:a=Vl}={}){const n=+getComputedStyle(t).opacity;return{delay:e,duration:r,easing:a,css:i=>`opacity: ${i*n}`}}function tr(t,{delay:e=0,duration:r=400,easing:a=bi,x:n=0,y:i=0,opacity:l=0}={}){const c=getComputedStyle(t),d=+c.opacity,f=c.transform==="none"?"":c.transform,h=d*(1-l),[p,g]=Ws(n),[b,w]=Ws(i);return{delay:e,duration:r,easing:a,css:(L,k)=>`
			transform: ${f} translate(${(1-L)*p}${g}, ${(1-L)*b}${w});
			opacity: ${d-h*k}`}}function pa(t,{delay:e=0,duration:r=400,easing:a=bi,start:n=0,opacity:i=0}={}){const l=getComputedStyle(t),c=+l.opacity,d=l.transform==="none"?"":l.transform,f=1-n,h=c*(1-i);return{delay:e,duration:r,easing:a,css:(p,g)=>`
			transform: ${d} scale(${1-f*g});
			opacity: ${c-h*g}
		`}}const Ql="modulepreload",Ul=function(t){return"/"+t},Bs={},Wl=function(e,r,a){let n=Promise.resolve();if(r&&r.length>0){let l=function(f){return Promise.all(f.map(h=>Promise.resolve(h).then(p=>({status:"fulfilled",value:p}),p=>({status:"rejected",reason:p}))))};document.getElementsByTagName("link");const c=document.querySelector("meta[property=csp-nonce]"),d=c?.nonce||c?.getAttribute("nonce");n=l(r.map(f=>{if(f=Ul(f),f in Bs)return;Bs[f]=!0;const h=f.endsWith(".css"),p=h?'[rel="stylesheet"]':"";if(document.querySelector(`link[href="${f}"]${p}`))return;const g=document.createElement("link");if(g.rel=h?"stylesheet":Ql,h||(g.as="script"),g.crossOrigin="",g.href=f,d&&g.setAttribute("nonce",d),document.head.appendChild(g),h)return new Promise((b,w)=>{g.addEventListener("load",b),g.addEventListener("error",()=>w(new Error(`Unable to preload CSS for ${f}`)))})}))}function i(l){const c=new Event("vite:preloadError",{cancelable:!0});if(c.payload=l,window.dispatchEvent(c),!c.defaultPrevented)throw l}return n.then(l=>{for(const c of l||[])c.status==="rejected"&&i(c.reason);return e().catch(i)})};function He(){return typeof window<"u"&&!!window.__TAURI_INTERNALS__}async function De(t,e){if(!He())throw new Error(`Tauri runtime not available for command "${t}"`);const{invoke:r}=await Wl(async()=>{const{invoke:a}=await Promise.resolve().then(()=>Bl);return{invoke:a}},void 0);return r(t,e)}const Te={loadInitialState:()=>De("load_initial_state"),saveSession:t=>De("save_session",{session:t}),saveFavorites:t=>De("save_favorites_cmd",{favorites:t}),saveRecentTracks:t=>De("save_recent_tracks_cmd",{tracks:t}),saveUserPlaylists:t=>De("save_user_playlists_cmd",{playlists:t}),saveSearchHistory:t=>De("save_search_history_cmd",{history:t}),getHomeFeed:(t=!1)=>De("get_home_feed",{forceRefresh:t}),getGenres:(t=!1)=>De("get_genres",{forceRefresh:t}),getGenrePlaylists:t=>De("get_genre_playlists",{genreId:t}),searchAll:t=>De("search_all",{query:t}),getSearchSuggestions:t=>De("get_search_suggestions",{query:t}),getArtist:(t,e)=>De("get_artist",{artistId:t,fallbackName:e}),getAlbumTracks:t=>De("get_album_tracks",{albumId:t}),getPlaylistTracks:t=>De("get_playlist_tracks",{playlistId:t}),getRadioTracks:t=>De("get_radio_tracks",{trackId:t}),getLyrics:(t,e,r)=>De("get_lyrics",{title:t,artist:e,durationSecs:r??null}),resolveAudioStream:t=>De("resolve_audio_stream",{track:t}),prefetchAudio:t=>De("prefetch_audio",{track:t}),cancelStaleStreams:t=>De("cancel_stale_streams",{keepIds:t}),getCacheBytes:()=>De("get_cache_bytes"),clearAudioCache:()=>De("clear_audio_cache"),spinStatus:()=>De("spin_status"),spinStartLocal:()=>De("spin_start_local"),spinStopLocal:()=>De("spin_stop_local"),spinSetAllowControls:t=>De("spin_set_allow_controls",{allow:t}),spinStartRemoteHost:()=>De("spin_start_remote_host"),spinStopRemote:()=>De("spin_stop_remote"),spinJoinRemote:t=>De("spin_join_remote",{code:t}),spinSyncSnapshot:t=>De("spin_sync_snapshot",{isPlaying:t.isPlaying,positionSecs:t.positionSecs,sourceName:t.sourceName,currentIndex:t.currentIndex,queue:t.queue})};function mi(t,e=!1){return window.__TAURI_INTERNALS__.transformCallback(t,e)}async function ys(t,e={},r){return window.__TAURI_INTERNALS__.invoke(t,e,r)}const Bl=Object.freeze(Object.defineProperty({__proto__:null,invoke:ys,transformCallback:mi},Symbol.toStringTag,{value:"Module"}));var Gs;(function(t){t.WINDOW_RESIZED="tauri://resize",t.WINDOW_MOVED="tauri://move",t.WINDOW_CLOSE_REQUESTED="tauri://close-requested",t.WINDOW_DESTROYED="tauri://destroyed",t.WINDOW_FOCUS="tauri://focus",t.WINDOW_BLUR="tauri://blur",t.WINDOW_SCALE_FACTOR_CHANGED="tauri://scale-change",t.WINDOW_THEME_CHANGED="tauri://theme-changed",t.WINDOW_CREATED="tauri://window-created",t.WINDOW_SUSPENDED="tauri://suspended",t.WINDOW_RESUMED="tauri://resumed",t.WEBVIEW_CREATED="tauri://webview-created",t.DRAG_ENTER="tauri://drag-enter",t.DRAG_OVER="tauri://drag-over",t.DRAG_DROP="tauri://drag-drop",t.DRAG_LEAVE="tauri://drag-leave"})(Gs||(Gs={}));async function Gl(t,e){window.__TAURI_EVENT_PLUGIN_INTERNALS__.unregisterListener(t,e),await ys("plugin:event|unlisten",{event:t,eventId:e})}async function ja(t,e,r){var a;const n=(a=void 0)!==null&&a!==void 0?a:{kind:"Any"};return ys("plugin:event|listen",{event:t,target:n,handler:mi(e)}).then(i=>async()=>Gl(t,i))}class Yl{decks;activeIndex=0;volume=1;crossfadeSecs=0;crossfadeFired=!1;fadeRaf=null;loading=!1;callbacks;constructor(e){this.callbacks=e,this.decks=[this.createDeck(0),this.createDeck(1)]}createDeck(e){const r=document.createElement("audio");return r.preload="auto",r.addEventListener("timeupdate",()=>{if(e!==this.activeIndex)return;const a=Number.isFinite(r.currentTime)?r.currentTime:0,n=Number.isFinite(r.duration)?r.duration:0;this.callbacks.onTimeUpdate(a,n),this.crossfadeSecs>0&&!this.crossfadeFired&&n>this.crossfadeSecs+2&&n-a<=this.crossfadeSecs&&(this.crossfadeFired=!0,this.callbacks.onCrossfadeTrigger())}),r.addEventListener("play",()=>{e===this.activeIndex&&this.callbacks.onPlayStateChange(!0)}),r.addEventListener("pause",()=>{e===this.activeIndex&&this.callbacks.onPlayStateChange(!1)}),r.addEventListener("ended",()=>{e===this.activeIndex&&this.callbacks.onEnded()}),r.addEventListener("error",()=>{if(e===this.activeIndex&&r.src&&!this.loading){const a=r.error?.code,n=r.error?.message||(a?`Audio error (code ${a})`:"Audio playback error");console.error("[GyroTube audio error]",{src:r.src,code:a,message:r.error?.message,networkState:r.networkState,readyState:r.readyState}),this.callbacks.onError(n)}}),r}cancelFade(){this.fadeRaf!==null&&(cancelAnimationFrame(this.fadeRaf),this.fadeRaf=null)}get activeDeck(){return this.decks[this.activeIndex]}setVolume(e){this.volume=Math.max(0,Math.min(1,e)),this.activeDeck.volume=this.volume}setCrossfadeSecs(e){this.crossfadeSecs=Math.max(0,Math.min(10,Math.round(e)))}async loadAndPlay(e,r=0,a=!1){this.cancelFade(),this.crossfadeFired=!1,this.loading=!0;const n=this.activeIndex,i=a?n===0?1:0:n,l=this.decks[n],c=this.decks[i];if(this.activeIndex=i,c.src=e,c.load(),r>0){const d=r;c.addEventListener("loadedmetadata",()=>{c.src===e&&Number.isFinite(d)&&(c.currentTime=d)},{once:!0})}try{if(a&&this.crossfadeSecs>0&&n!==i){const d=l.volume;c.volume=.01,await c.play();const f=this.crossfadeSecs*1e3,h=performance.now(),p=g=>{const b=Math.min(1,(g-h)/f);l.volume=Math.max(0,d*(1-b)),c.volume=Math.min(1,this.volume*b),b<1&&this.activeIndex===i?this.fadeRaf=requestAnimationFrame(p):(this.fadeRaf=null,this.activeIndex!==n&&(l.pause(),l.removeAttribute("src"),l.load()))};this.fadeRaf=requestAnimationFrame(p)}else n!==i&&(l.pause(),l.removeAttribute("src"),l.load()),c.volume=this.volume,await c.play()}catch(d){throw console.error("[GyroTube play() failed]",{url:e,err:d,canPlayWebm:c.canPlayType("audio/webm"),canPlayMp4:c.canPlayType("audio/mp4"),errorCode:c.error?.code,errorMessage:c.error?.message}),d}finally{this.loading=!1}}async togglePlay(){const e=this.activeDeck;e.src&&(e.paused?await e.play():e.pause())}seek(e){const r=this.activeDeck;!r.src||!Number.isFinite(e)||(r.currentTime=Math.max(0,e))}stop(){this.cancelFade();for(const e of this.decks)e.pause(),e.removeAttribute("src"),e.load();this.callbacks.onPlayStateChange(!1),this.callbacks.onTimeUpdate(0,0)}updateMediaSession(e,r){if(!("mediaSession"in navigator))return;if(!e){navigator.mediaSession.metadata=null;return}const a=e.cover_large_url||e.cover_url;navigator.mediaSession.metadata=new MediaMetadata({title:e.title,artist:e.artist,album:e.album??"",artwork:a?[{src:a,sizes:"512x512",type:"image/jpeg"}]:[]}),navigator.mediaSession.setActionHandler("play",r.onPlay),navigator.mediaSession.setActionHandler("pause",r.onPause),navigator.mediaSession.setActionHandler("previoustrack",r.onPrev),navigator.mediaSession.setActionHandler("nexttrack",r.onNext),navigator.mediaSession.setActionHandler("seekto",n=>{n.seekTime!=null&&r.onSeek(n.seekTime)})}}async function Ys(t){return new Promise(e=>{const r=new Image;r.crossOrigin="anonymous",r.referrerPolicy="no-referrer",r.onload=()=>{try{const a=document.createElement("canvas");a.width=24,a.height=24;const n=a.getContext("2d");if(!n)return e(null);n.drawImage(r,0,0,24,24);const i=n.getImageData(0,0,24,24).data;let l=0,c=0,d=0,f=0;for(let h=0;h<i.length;h+=4){const p=i[h],g=i[h+1],b=i[h+2],w=Math.max(p,g,b),L=Math.min(p,g,b),k=w===0?0:(w-L)/w,M=(w+L)/510;if(M<.08||M>.92)continue;const A=.25+k*2.5;l+=p*A,c+=g*A,d+=b*A,f+=A}if(f===0)return e(null);e([Math.round(l/f),Math.round(c/f),Math.round(d/f)])}catch{e(null)}},r.onerror=()=>e(null),r.src=t})}class Kl{#r=O("home");get activeTab(){return s(this.#r)}set activeTab(e){x(this.#r,e,!0)}#l=O(Ke({kind:"home"}));get activeView(){return s(this.#l)}set activeView(e){x(this.#l,e,!0)}#e=O(!0);get sidebarOpen(){return s(this.#e)}set sidebarOpen(e){x(this.#e,e,!0)}#i=O("player");get sidebarTab(){return s(this.#i)}set sidebarTab(e){x(this.#i,e,!0)}#a=O(!1);get spinOpen(){return s(this.#a)}set spinOpen(e){x(this.#a,e,!0)}#t=O(!1);get partyMode(){return s(this.#t)}set partyMode(e){x(this.#t,e,!0)}#n=O(!1);get settingsOpen(){return s(this.#n)}set settingsOpen(e){x(this.#n,e,!0)}#s=O(Ke([]));get queue(){return s(this.#s)}set queue(e){x(this.#s,e,!0)}#o=O(Ke([]));get unshuffledQueue(){return s(this.#o)}set unshuffledQueue(e){x(this.#o,e,!0)}#f=O(null);get currentIndex(){return s(this.#f)}set currentIndex(e){x(this.#f,e,!0)}#c=O("Not playing");get sourceName(){return s(this.#c)}set sourceName(e){x(this.#c,e,!0)}#h=O(!1);get isPlaying(){return s(this.#h)}set isPlaying(e){x(this.#h,e,!0)}#g=O(!1);get isLoadingAudio(){return s(this.#g)}set isLoadingAudio(e){x(this.#g,e,!0)}#u=O(0);get positionSecs(){return s(this.#u)}set positionSecs(e){x(this.#u,e,!0)}#p=O(0);get durationSecs(){return s(this.#p)}set durationSecs(e){x(this.#p,e,!0)}#m=O("YTM");get qualityLabel(){return s(this.#m)}set qualityLabel(e){x(this.#m,e,!0)}#w=O(1);get volume(){return s(this.#w)}set volume(e){x(this.#w,e,!0)}#b=O(!1);get shuffle(){return s(this.#b)}set shuffle(e){x(this.#b,e,!0)}#d=O(!1);get repeat(){return s(this.#d)}set repeat(e){x(this.#d,e,!0)}#_=O(0);get crossfadeSecs(){return s(this.#_)}set crossfadeSecs(e){x(this.#_,e,!0)}#x=O(!0);get autoplayRelated(){return s(this.#x)}set autoplayRelated(e){x(this.#x,e,!0)}#v=O(!0);get backgroundPlayback(){return s(this.#v)}set backgroundPlayback(e){x(this.#v,e,!0)}#y=O("system");get themeMode(){return s(this.#y)}set themeMode(e){x(this.#y,e,!0)}#k=O("Auto");get language(){return s(this.#k)}set language(e){x(this.#k,e,!0)}#$=O(null);get ambientRgb(){return s(this.#$)}set ambientRgb(e){x(this.#$,e,!0)}#S=O(Ke({type:"None"}));get lyrics(){return s(this.#S)}set lyrics(e){x(this.#S,e,!0)}#T=O(null);get activeLyricIndex(){return s(this.#T)}set activeLyricIndex(e){x(this.#T,e,!0)}#P=O(Ke([]));get suggestedTracks(){return s(this.#P)}set suggestedTracks(e){x(this.#P,e,!0)}#A=O(Ke([]));get favorites(){return s(this.#A)}set favorites(e){x(this.#A,e,!0)}#E=O(Ke([]));get recentTracks(){return s(this.#E)}set recentTracks(e){x(this.#E,e,!0)}#M=O(Ke([]));get userPlaylists(){return s(this.#M)}set userPlaylists(e){x(this.#M,e,!0)}#I=O(Ke([]));get searchHistory(){return s(this.#I)}set searchHistory(e){x(this.#I,e,!0)}#N=O(null);get homeFeed(){return s(this.#N)}set homeFeed(e){x(this.#N,e,!0)}#z=O(!1);get homeLoading(){return s(this.#z)}set homeLoading(e){x(this.#z,e,!0)}#L=O(Ke([]));get genres(){return s(this.#L)}set genres(e){x(this.#L,e,!0)}#C=O("");get searchQuery(){return s(this.#C)}set searchQuery(e){x(this.#C,e,!0)}#R=O(null);get searchResults(){return s(this.#R)}set searchResults(e){x(this.#R,e,!0)}#q=O(!1);get searchLoading(){return s(this.#q)}set searchLoading(e){x(this.#q,e,!0)}#O=O(Ke({local_active:!1,local_url:"",local_qr_svg:null,allow_controls:!1,guest_count:0,added_count:0,remote_role:"idle",remote_ticket:null,remote_qr_svg:null}));get spinStatus(){return s(this.#O)}set spinStatus(e){x(this.#O,e,!0)}#j=O(Ke([]));get toasts(){return s(this.#j)}set toasts(e){x(this.#j,e,!0)}toastSeq=0;loadGen=0;engine;#F=Z(()=>this.currentIndex!=null&&this.currentIndex>=0&&this.currentIndex<this.queue.length?this.queue[this.currentIndex]:null);get currentTrack(){return s(this.#F)}set currentTrack(e){x(this.#F,e)}#D=Z(()=>this.currentIndex!=null&&this.currentIndex+1<this.queue.length?this.queue[this.currentIndex+1]:null);get nextTrack(){return s(this.#D)}set nextTrack(e){x(this.#D,e)}constructor(){this.engine=new Yl({onTimeUpdate:(e,r)=>{this.positionSecs=e,r>0&&(this.durationSecs=r),this.updateActiveLyric(e)},onPlayStateChange:e=>{this.isPlaying=e,e&&(this.isLoadingAudio=!1),this.syncSpinSnapshot()},onEnded:()=>{this.handleTrackEnded()},onCrossfadeTrigger:()=>{this.next(!0)},onError:e=>{this.isLoadingAudio=!1,this.showToast(e)}})}async bootstrap(){if(this.applyTheme(this.themeMode),!!He()){try{const e=await Te.loadInitialState();this.favorites=e.favorites,this.recentTracks=e.recent_tracks,this.userPlaylists=e.user_playlists,this.searchHistory=e.search_history;const r=e.session;this.sidebarOpen=!r.sidebar_collapsed,this.queue=r.queue,this.unshuffledQueue=r.unshuffled_queue,this.currentIndex=r.current_index,this.sourceName=r.source_name||"Not playing",this.volume=r.user_volume??1,this.shuffle=r.shuffle,this.repeat=r.repeat,this.crossfadeSecs=r.crossfade_secs??0,this.autoplayRelated=r.autoplay_related??!0,this.backgroundPlayback=r.background_playback??!0,this.language=r.language??"Auto",this.themeMode=r.theme_mode||"system",this.engine.setVolume(this.volume),this.engine.setCrossfadeSecs(this.crossfadeSecs),this.applyTheme(this.themeMode);const a=this.currentTrack;if(a){this.durationSecs=a.duration_secs??0;const n=a.cover_large_url||a.cover_url;n&&Ys(n).then(i=>{this.currentTrack?.id===a.id&&(this.ambientRgb=i)}),Te.getLyrics(a.title,a.artist,a.duration_secs).then(i=>{this.currentTrack?.id===a.id&&(this.lyrics=i)})}}catch(e){this.showToast(String(e))}this.loadHome(),this.loadGenres(),this.refreshSpinStatus(),this.bindNativeEvents()}}async bindNativeEvents(){if(He())try{await ja("tray:action",e=>{e.payload==="toggle_play"?this.togglePlay():e.payload==="next"?this.next():e.payload==="prev"&&this.prev()}),await ja("spin:command",e=>{const r=e.payload;r.kind==="toggle_play_pause"?this.togglePlay():r.kind==="next_track"?this.next():r.kind==="prev_track"?this.prev():r.kind==="move_track"?this.moveQueueItem(r.from,r.to):r.kind==="add_track"&&Te.searchAll(r.video_id).then(a=>{const n=a.tracks.find(i=>i.id===r.video_id)??a.tracks[0];n&&(this.enqueueTrack(n),this.showToast(`${r.guest_name} added ${n.title}`))})}),await ja("spin:remote-sync",e=>{const{payload:r,error:a}=e.payload;if(a){this.showToast(a);return}if(!r?.track)return;const n=Math.max(0,Date.now()-r.sent_at_ms),i=r.position_secs+(r.is_playing?n/1e3:0);this.currentTrack?.id!==r.track.id?(this.queue=[r.track],this.unshuffledQueue=[r.track],this.sourceName=`SPIN · ${r.host_name||"Host"}`,this.playIndex(0,!1,i)):(Math.abs(this.positionSecs-i)>1.8&&this.seek(i),r.is_playing!==this.isPlaying&&this.engine.togglePlay())})}catch{}}applyTheme(e){if(this.themeMode=e,typeof document>"u")return;const r=document.documentElement;e==="system"?r.removeAttribute("data-theme"):r.setAttribute("data-theme",e)}setThemeMode(e){this.applyTheme(e),this.persistSession()}setCrossfadeSecs(e){this.crossfadeSecs=Math.max(0,Math.min(10,Math.round(e))),this.engine.setCrossfadeSecs(this.crossfadeSecs),this.persistSession()}setAutoplay(e){this.autoplayRelated=e,this.persistSession()}setBackgroundPlayback(e){this.backgroundPlayback=e,this.persistSession()}setVolume(e){this.volume=Math.max(0,Math.min(1,e)),this.engine.setVolume(this.volume),this.persistSession()}showToast(e){const r=++this.toastSeq;this.toasts=[...this.toasts,{id:r,message:e}],window.setTimeout(()=>{this.toasts=this.toasts.filter(a=>a.id!==r)},3200)}navigateTab(e){e==="explore"&&this.activeTab==="explore"&&this.activeView.kind==="explore"&&this.clearSearch(),this.activeTab=e,this.activeView={kind:e}}clearSearch(){this.searchQuery="",this.searchResults=null,this.searchLoading=!1}openArtist(e,r){this.activeTab="explore",this.activeView={kind:"artist",artistId:e,artistName:r}}openAlbum(e,r,a,n=!1){this.activeView={kind:"album",id:e,title:r,coverUrl:a,isPlaylist:n}}async loadHome(e=!1){if(He()){this.homeLoading=!0;try{this.homeFeed=await Te.getHomeFeed(e)}catch(r){this.showToast(String(r))}finally{this.homeLoading=!1}}}async loadGenres(e=!1){if(He())try{this.genres=await Te.getGenres(e)}catch{}}async runSearch(e){const r=e.trim();if(r&&(this.searchQuery=r,this.activeTab="explore",this.activeView={kind:"explore"},this.searchHistory=[r,...this.searchHistory.filter(a=>a.toLowerCase()!==r.toLowerCase())].slice(0,8),He())){Te.saveSearchHistory(this.searchHistory),this.searchLoading=!0;try{this.searchResults=await Te.searchAll(r)}catch(a){this.showToast(String(a))}finally{this.searchLoading=!1}}}clearSearchHistory(){this.searchHistory=[],He()&&Te.saveSearchHistory([])}isFavorite(e){return this.favorites.some(r=>r.id===e)}toggleFavorite(e){this.isFavorite(e.id)?this.favorites=this.favorites.filter(r=>r.id!==e.id):this.favorites=[e,...this.favorites],He()&&Te.saveFavorites(this.favorites)}replaceQueueAndPlay(e,r,a){e.length&&(this.queue=[...e],this.unshuffledQueue=[...e],this.sourceName=a,this.shuffle=!1,this.playIndex(Math.max(0,Math.min(r,e.length-1))))}enqueueTrack(e){this.queue=[...this.queue,e],this.showToast(`Added to queue: ${e.title}`),this.currentIndex==null?this.playIndex(0):this.persistSession()}async startRadioFromCurrent(){const e=this.currentTrack;if(!(!e||!He()))try{const a=(await Te.getRadioTracks(e.id)).filter(n=>n.id!==e.id);if(a.length===0){this.showToast("No related radio tracks found");return}this.queue=[e,...a],this.unshuffledQueue=[...this.queue],this.currentIndex=0,this.shuffle=!1,this.sourceName=`Radio · ${e.title}`,this.persistSession(),this.syncSpinSnapshot(),this.showToast(`Radio started · ${a.length} songs queued`)}catch(r){this.showToast(String(r))}}removeQueueIndex(e){if(e<0||e>=this.queue.length)return;const r=this.queue.filter((a,n)=>n!==e);if(this.queue=r,this.currentIndex!=null){if(r.length===0)this.currentIndex=null,this.engine.stop();else if(e===this.currentIndex){const a=Math.min(e,r.length-1);this.playIndex(a)}else e<this.currentIndex&&(this.currentIndex-=1);this.persistSession()}}moveQueueItem(e,r){if(e===r||e<0||r<0||e>=this.queue.length||r>=this.queue.length)return;const a=[...this.queue],[n]=a.splice(e,1);a.splice(r,0,n),this.queue=a,this.currentIndex===e?this.currentIndex=r:this.currentIndex!=null&&(e<this.currentIndex&&r>=this.currentIndex?this.currentIndex-=1:e>this.currentIndex&&r<=this.currentIndex&&(this.currentIndex+=1)),this.persistSession()}clearQueue(){this.queue=[],this.unshuffledQueue=[],this.currentIndex=null,this.engine.stop(),this.lyrics={type:"None"},this.ambientRgb=null,this.persistSession()}async playIndex(e,r=!1,a=0){if(e<0||e>=this.queue.length)return;const n=this.queue[e],i=++this.loadGen;this.currentIndex=e,this.isLoadingAudio=!0,this.positionSecs=0,this.durationSecs=n.duration_secs??0,this.lyrics={type:"None"},this.activeLyricIndex=null,this.recentTracks=[n,...this.recentTracks.filter(c=>c.id!==n.id)].slice(0,30),He()&&Te.saveRecentTracks(this.recentTracks);const l=n.cover_large_url||n.cover_url;if(l?Ys(l).then(c=>{this.loadGen===i&&(this.ambientRgb=c)}):this.ambientRgb=null,this.engine.updateMediaSession(n,{onPlay:()=>void this.togglePlay(),onPause:()=>void this.togglePlay(),onPrev:()=>void this.prev(),onNext:()=>void this.next(),onSeek:c=>this.seek(c)}),this.persistSession(),!He()){this.isLoadingAudio=!1;return}try{Te.cancelStaleStreams([n.id]);const c=await Te.resolveAudioStream(n);if(this.loadGen!==i)return;this.qualityLabel=c.quality_label,await this.engine.loadAndPlay(c.url,a,r),this.syncSpinSnapshot(),Te.getLyrics(n.title,n.artist,n.duration_secs).then(d=>{this.loadGen===i&&(this.lyrics=d)}),Te.getRadioTracks(n.id).then(d=>{if(this.loadGen===i){const f=new Set(this.queue.map(h=>h.id));this.suggestedTracks=d.filter(h=>!f.has(h.id)).slice(0,12)}})}catch(c){this.loadGen===i&&(this.isLoadingAudio=!1,this.showToast(String(c)))}}async togglePlay(){if(!(!this.currentTrack||this.currentIndex==null)){if(!this.engine.activeDeck.src){await this.playIndex(this.currentIndex);return}await this.engine.togglePlay()}}seek(e){this.positionSecs=e,this.engine.seek(e),this.updateActiveLyric(e)}async prev(){if(this.positionSecs>3){this.seek(0);return}this.currentIndex!=null&&this.currentIndex>0&&await this.playIndex(this.currentIndex-1)}async next(e=!1){if(this.currentIndex!=null){if(this.currentIndex+1<this.queue.length)await this.playIndex(this.currentIndex+1,e);else if(this.repeat&&this.queue.length>0)await this.playIndex(0,e);else if(this.autoplayRelated&&this.suggestedTracks.length>0){const r=this.suggestedTracks[0];this.suggestedTracks=this.suggestedTracks.slice(1),this.queue=[...this.queue,r],await this.playIndex(this.queue.length-1,e)}}}async handleTrackEnded(){this.currentIndex!=null&&(this.currentIndex+1<this.queue.length||this.repeat||this.autoplayRelated)&&await this.next(!1)}toggleShuffle(){if(this.shuffle=!this.shuffle,this.queue.length<=1||this.currentIndex==null)return;const e=this.queue[this.currentIndex];if(this.shuffle){this.unshuffledQueue=[...this.queue];const r=this.queue.filter((a,n)=>n!==this.currentIndex);for(let a=r.length-1;a>0;a--){const n=Math.floor(Math.random()*(a+1));[r[a],r[n]]=[r[n],r[a]]}this.queue=[e,...r],this.currentIndex=0}else if(this.unshuffledQueue.length>0){const r=this.unshuffledQueue.findIndex(a=>a.id===e.id);this.queue=[...this.unshuffledQueue],this.currentIndex=r>=0?r:0}this.persistSession()}toggleRepeat(){this.repeat=!this.repeat,this.persistSession()}updateActiveLyric(e){if(this.lyrics.type!=="Synced")return;const r=this.lyrics.lines;let a=null;for(let n=r.length-1;n>=0;n--)if(r[n].time_secs<=e+.15){a=n;break}a!==this.activeLyricIndex&&(this.activeLyricIndex=a)}async refreshSpinStatus(){if(He())try{this.spinStatus=await Te.spinStatus(),this.syncSpinSnapshot()}catch{}}async syncSpinSnapshot(){if(He()&&!(!this.spinStatus.local_active&&this.spinStatus.remote_role!=="host"))try{await Te.spinSyncSnapshot({isPlaying:this.isPlaying,positionSecs:this.positionSecs,sourceName:this.sourceName,currentIndex:this.currentIndex,queue:this.queue})}catch{}}createUserPlaylist(e,r="",a="#2190a4",n=null){const i=e.trim()||"New playlist",l={id:`pl-${Date.now()}-${Math.random().toString(36).slice(2,7)}`,name:i,description:r.trim(),color_hex:a,icon_name:n,cover_path:null,tracks:[]};return this.userPlaylists=[l,...this.userPlaylists],He()&&Te.saveUserPlaylists(this.userPlaylists),l}updateUserPlaylist(e,r,a,n,i){const l=r.trim();l&&(this.userPlaylists=this.userPlaylists.map(c=>c.id===e?{...c,name:l,description:a.trim(),color_hex:n,icon_name:i}:c),He()&&Te.saveUserPlaylists(this.userPlaylists))}deleteUserPlaylist(e){this.userPlaylists=this.userPlaylists.filter(r=>r.id!==e),He()&&Te.saveUserPlaylists(this.userPlaylists)}addTrackToPlaylist(e,r){let a=!1;this.userPlaylists=this.userPlaylists.map(n=>n.id!==e||n.tracks.some(i=>i.id===r.id)?n:(a=!0,{...n,tracks:[...n.tracks,r]})),a&&(He()&&Te.saveUserPlaylists(this.userPlaylists),this.showToast(`Added "${r.title}" to playlist`))}removeTrackFromPlaylist(e,r){this.userPlaylists=this.userPlaylists.map(a=>a.id===e?{...a,tracks:a.tracks.filter(n=>n.id!==r)}:a),He()&&Te.saveUserPlaylists(this.userPlaylists)}async persistSession(){if(!He())return;const e={sidebar_collapsed:!this.sidebarOpen,queue:this.queue,unshuffled_queue:this.unshuffledQueue,current_index:this.currentIndex,source_name:this.sourceName,user_volume:this.volume,shuffle:this.shuffle,repeat:this.repeat,position_secs:this.positionSecs,crossfade_secs:this.crossfadeSecs,language:this.language,background_playback:this.backgroundPlayback,autoplay_related:this.autoplayRelated,theme_mode:this.themeMode};try{await Te.saveSession(e)}catch{}}}const o=new Kl;function St(t){if(t==null||!Number.isFinite(t)||t<0)return"--:--";const e=Math.floor(t),r=Math.floor(e/60),a=e%60;return`${r}:${a.toString().padStart(2,"0")}`}function _a(t){return t?t>=1e6?`${(t/1e6).toFixed(1)} M subscribers`:t>=1e3?`${(t/1e3).toFixed(1)} K subscribers`:`${t} subscribers`:""}var Jl=S('<div class="cover-skeleton svelte-1gzalhk" aria-hidden="true"></div>'),Xl=S('<img decoding="async" referrerpolicy="no-referrer"/>'),Zl=S("<!> <!>",1),ec=S('<div class="cover-fallback svelte-1gzalhk" aria-hidden="true"> </div>'),tc=S("<div><!></div>");function Ze(t,e){Je(e,!0);let r=ft(e,"src",3,null),a=ft(e,"fallbackSrc",3,null),n=ft(e,"alt",3,""),i=ft(e,"class",3,""),l=ft(e,"fallbackIcon",3,"♪"),c=ft(e,"eager",3,!1),d=O(null),f=O(!1),h=O(!1),p=O(0),g=O(!1),b=O(null);function w(){return e.targetPx?e.targetPx:i().includes("track-cover")||i().includes("tile-cover")||i().includes("up-next-cover")||i().includes("w-10")?120:i().includes("media-cover")||i().includes("artist-avatar")||i().includes("top-avatar")||i().includes("hero-cover")||i().includes("album-cover")?360:544}function L(C){if(!C)return null;const $=w();return C.includes("googleusercontent.com")&&/=w\d+-h\d+/.test(C)?C.replace(/=w\d+-h\d+[^&?]*/,`=w${$}-h${$}-l90-rj`):C.includes("i.ytimg.com")&&$<=120?C.replace(/\/(maxresdefault|sddefault|hqdefault)\./,"/mqdefault."):C}At(()=>{const C=L(r())||L(a())||null;C!==s(b)&&(x(b,C,!0),x(p,0),x(g,!1),x(h,!1))}),Sa(()=>{if(c()){x(f,!0);return}if(!s(d)||typeof IntersectionObserver>"u"){x(f,!0);return}const C=new IntersectionObserver($=>{$.some(m=>m.isIntersecting)&&(x(f,!0),C.disconnect())},{rootMargin:"160px"});return C.observe(s(d)),()=>C.disconnect()});function k(){if(s(p)===0&&a()){const C=L(a());if(C&&C!==s(b)){x(p,1),x(b,C,!0);return}}if(s(p)<2&&s(b)){if(x(p,2),s(b).includes("googleusercontent.com")&&s(b).includes("=w")){const C=s(b).replace(/=w\d+-h\d+[^&?]*/,"=w226-h226-l90-rj");if(C!==s(b)){x(b,C,!0);return}}if(s(b).includes("i.ytimg.com")&&!s(b).includes("hqdefault")){const C=s(b).replace(/\/(maxresdefault|sddefault|mqdefault)\./,"/hqdefault.");if(C!==s(b)){x(b,C,!0);return}}}x(g,!0)}var M=tc(),A=v(M);{var z=C=>{var $=Zl(),m=se($);{var P=j=>{var R=Jl();_(j,R)};F(m,j=>{s(h)||j(P)})}var D=u(m,2);{var T=j=>{var R=Xl();U(()=>{X(R,1,`cover-img ${s(h)?"is-loaded":""}`,"svelte-1gzalhk"),it(R,"src",s(b)),it(R,"alt",n()),it(R,"loading",c()?"eager":"lazy")}),xt("load",R,()=>x(h,!0)),xt("error",R,k),_(j,R)};F(D,j=>{(s(f)||c())&&j(T)})}_(C,$)},q=C=>{var $=ec(),m=N($,!0);U(()=>I(m,l())),_(C,$)};F(A,C=>{s(b)&&!s(g)?C(z):C(q,-1)})}br(M,C=>x(d,C),()=>s(d)),U(()=>X(M,1,`cover-shell ${i()??""}`,"svelte-1gzalhk")),_(t,M),Xe()}var rc=S('<p class="artist svelte-hawz4i"> </p>'),ac=S('<p class="muted">Loading tracks…</p>'),sc=S('<div role="button" tabindex="0"><span> </span> <div class="track-meta"><span class="track-title"> </span> <span class="track-sub"> </span></div> <span class="muted numeric dur svelte-hawz4i"> </span> <button>♥</button> <button class="icon-btn">+</button></div>'),nc=S('<div class="track-list"></div>'),ic=S('<div class="page-clamp"><div class="album-hero svelte-hawz4i"><button class="icon-btn back-btn svelte-hawz4i" title="Back">←</button> <div class="hero-row svelte-hawz4i"><!> <div class="album-meta svelte-hawz4i"><span class="type-pill svelte-hawz4i"> </span> <h1 class="svelte-hawz4i"> </h1> <!> <p class="muted small svelte-hawz4i"> </p></div></div></div> <div class="album-actions svelte-hawz4i"><button class="play-circle" title="Play all">▶</button> <button class="pill-btn">⇄ Shuffle</button></div> <!></div>');function oc(t,e){Je(e,!0);let r=ft(e,"coverUrl",3,null),a=ft(e,"isPlaylist",3,!1),n=O(Ke([])),i=O(""),l=O(!1);At(()=>{const R=e.id,G=a();if(x(i,e.title,!0),x(n,[],!0),!He())return;x(l,!0),(G?Te.getPlaylistTracks(R):Te.getAlbumTracks(R)).then(([K,ce])=>{e.id===R&&(K&&x(i,K,!0),x(n,ce,!0))}).catch(K=>o.showToast(String(K))).finally(()=>{e.id===R&&x(l,!1)})});let c=Z(()=>Math.round(s(n).reduce((R,G)=>R+(G.duration_secs??0),0)/60));var d=ic(),f=v(d),h=v(f),p=u(h,2),g=v(p);{let R=Z(()=>s(n)[0]?.cover_url);Ze(g,{class:"album-cover",get src(){return r()},get fallbackSrc(){return s(R)},get alt(){return s(i)}})}var b=u(g,2),w=v(b),L=N(w,!0),k=u(w,2),M=N(k,!0),A=u(k,2);{var z=R=>{var G=rc(),H=N(G,!0);U(()=>I(H,s(n)[0].artist)),_(R,G)};F(A,R=>{s(n)[0]?.artist&&R(z)})}var q=u(A,2),C=N(q),$=u(f,2),m=v($),P=u(m,2),D=u($,2);{var T=R=>{var G=ac();_(R,G)},j=R=>{var G=nc();Ve(G,23,()=>s(n),(H,K)=>H.id+":"+K,(H,K,ce)=>{const ne=Z(()=>o.currentTrack?.id===s(K).id);var Y=sc();let ue;var le=v(Y);let ge;var V=N(le,!0),ie=u(le,2),W=v(ie),ee=N(W,!0),E=u(W,2),J=N(E,!0),te=u(ie,2),Le=N(te,!0),he=u(te,2);let Oe;var re=u(he,2);U((ve,Ie)=>{ue=X(Y,1,"track-row",null,ue,{active:s(ne)}),ge=X(le,1,"num numeric svelte-hawz4i",null,ge,{"active-num":s(ne)}),I(V,s(ce)+1),I(ee,s(K).title),I(J,s(K).artist),I(Le,ve),Oe=X(he,1,"icon-btn",null,Oe,{"fav-active":Ie})},[()=>St(s(K).duration_secs),()=>o.isFavorite(s(K).id)]),y("click",Y,()=>o.replaceQueueAndPlay(s(n),s(ce),s(i))),y("keydown",Y,ve=>ve.key==="Enter"&&o.replaceQueueAndPlay(s(n),s(ce),s(i))),y("click",he,ve=>{ve.stopPropagation(),o.toggleFavorite(s(K))}),y("click",re,ve=>{ve.stopPropagation(),o.enqueueTrack(s(K))}),_(H,Y)}),_(R,G)};F(D,R=>{s(l)?R(T):s(n).length>0&&R(j,1)})}U(()=>{I(L,a()?"PLAYLIST":"ALBUM"),I(M,s(i)),I(C,`${s(n).length??""} songs${s(c)?` • ${s(c)} min`:""}`),m.disabled=s(n).length===0,P.disabled=s(n).length===0}),y("click",h,()=>o.navigateTab(o.activeTab)),y("click",m,()=>o.replaceQueueAndPlay(s(n),0,s(i))),y("click",P,()=>{const R=[...s(n)].sort(()=>Math.random()-.5);o.replaceQueueAndPlay(R,0,s(i))}),_(t,d),Xe()}at(["click","keydown"]);var lc=S('<div class="flex items-center gap-1.5"><button type="button" class="w-8 h-8 rounded-full inline-flex items-center justify-center bg-[var(--surface)] border border-[var(--border)] hover:bg-[var(--surface-hover)] hover:border-[var(--border-strong)] disabled:opacity-30 transition-all active:scale-95" title="Scroll left" aria-label="Scroll left"><!></button> <button type="button" class="w-8 h-8 rounded-full inline-flex items-center justify-center bg-[var(--surface)] border border-[var(--border)] hover:bg-[var(--surface-hover)] hover:border-[var(--border-strong)] disabled:opacity-30 transition-all active:scale-95" title="Scroll right" aria-label="Scroll right"><!></button></div>'),cc=S('<div class="flex items-center justify-between gap-2"><h2 class="section-title"> </h2> <!></div>'),uc=S('<button type="button" class="hidden md:inline-flex items-center justify-center w-9 h-9 rounded-full absolute left-2 top-1/2 -translate-y-1/2 bg-[var(--bg-elevated)]/90 backdrop-blur-md border border-[var(--border-strong)] shadow-xl opacity-0 group-hover:opacity-100 transition-all duration-200 hover:bg-[var(--accent)] hover:text-white hover:border-transparent active:scale-95" aria-label="Scroll left"><!></button>'),dc=S('<button type="button" class="hidden md:inline-flex items-center justify-center w-9 h-9 rounded-full absolute right-2 top-1/2 -translate-y-1/2 bg-[var(--bg-elevated)]/90 backdrop-blur-md border border-[var(--border-strong)] shadow-xl opacity-0 group-hover:opacity-100 transition-all duration-200 hover:bg-[var(--accent)] hover:text-white hover:border-transparent active:scale-95" aria-label="Scroll right"><!></button>'),vc=S('<section class="flex flex-col gap-3"><!> <div class="relative group"><div class="carousel-track scroll-smooth svelte-e7zbyu"><!></div> <!> <!></div></section>');function Sr(t,e){Je(e,!0);let r=ft(e,"minCardWidth",3,172),a=ft(e,"gap",3,12),n=O(null),i=O(!1),l=O(!1),c=O(5),d=O(176);function f(){if(!s(n))return;const $=s(n).clientWidth;if($>0){const m=Math.max(2,Math.floor(($+a())/(r()+a())));x(c,m,!0),x(d,($-a()*(m-1))/m)}h()}function h(){if(!s(n))return;const{scrollLeft:$,scrollWidth:m,clientWidth:P}=s(n);x(i,$>4),x(l,$+P<m-4)}function p($){if(!s(n))return;const m=s(c)*(s(d)+a());s(n).scrollBy({left:$*m,behavior:"smooth"})}Sa(()=>{if(f(),!s(n))return;const $=new ResizeObserver(()=>f());$.observe(s(n));const m=new MutationObserver(()=>h());return m.observe(s(n),{childList:!0}),()=>{$.disconnect(),m.disconnect()}});var g=vc(),b=v(g);{var w=$=>{var m=cc(),P=v(m),D=N(P,!0),T=u(P,2);{var j=R=>{var G=lc(),H=v(G),K=v(H);ts(K,{size:16});var ce=u(H,2),ne=v(ce);Hs(ne,{size:16}),U(()=>{H.disabled=!s(i),ce.disabled=!s(l)}),y("click",H,()=>p(-1)),y("click",ce,()=>p(1)),_(R,G)};F(T,R=>{(s(i)||s(l))&&R(j)})}U(()=>I(D,e.title)),_($,m)};F(b,$=>{e.title&&$(w)})}var L=u(b,2),k=v(L),M=v(k);rl(M,()=>e.children),br(k,$=>x(n,$),()=>s(n));var A=u(k,2);{var z=$=>{var m=uc(),P=v(m);ts(P,{size:18}),y("click",m,()=>p(-1)),_($,m)};F(A,$=>{s(i)&&$(z)})}var q=u(A,2);{var C=$=>{var m=dc(),P=v(m);Hs(P,{size:18}),y("click",m,()=>p(1)),_($,m)};F(q,$=>{s(l)&&$(C)})}U(()=>{it(g,"id",e.id),lt(k,`--carousel-card-w: ${s(d)??""}px; --carousel-gap: ${a()??""}px;`)}),xt("scroll",k,h),_(t,g),Xe()}at(["click"]);var fc=S('<img alt="" loading="lazy" decoding="async" referrerpolicy="no-referrer"/>'),hc=S('<p class="muted"> </p>'),pc=S('<p class="muted">Loading artist…</p>'),_c=S('<div role="button" tabindex="0"><!> <div class="track-meta"><span class="track-title"> </span> <span class="track-sub"> </span></div> <span class="muted numeric dur svelte-1iungmq"> </span> <button>♥</button> <button class="icon-btn">+</button></div>'),gc=S('<button class="pill-btn toggle-btn svelte-1iungmq"> </button>'),bc=S('<section class="section svelte-1iungmq"><h2 class="section-title">Popular songs</h2> <div class="track-list"></div> <!></section>'),mc=S('<button class="media-card svelte-1iungmq"><!> <strong class="media-title svelte-1iungmq"> </strong> <span class="media-sub muted svelte-1iungmq"> </span></button>'),Ks=S('<div class="section svelte-1iungmq"><!></div>'),xc=S('<span class="muted text-[11.5px]"> </span>'),yc=S('<button class="w-[148px] shrink-0 p-2.5 rounded-2xl flex flex-col items-center gap-1.5 text-center hover:bg-[var(--surface)] transition-colors"><!> <strong class="text-[13px] truncate w-full"> </strong> <!></button>'),wc=S('<section class="section svelte-1iungmq"><div class="bio-card svelte-1iungmq"><h3 class="svelte-1iungmq">About the artist</h3> <p class="muted svelte-1iungmq"> </p></div></section>'),kc=S("<!> <!> <!> <!>",1),$c=S('<div class="page-clamp"><div class="hero svelte-1iungmq"><!> <div class="hero-scrim svelte-1iungmq"><button class="icon-btn back-btn svelte-1iungmq" title="Back">←</button> <div class="hero-bottom svelte-1iungmq"><span class="verified svelte-1iungmq">✓ Artist</span> <h1 class="svelte-1iungmq"> </h1> <!> <div class="hero-actions svelte-1iungmq"><button class="pill-btn primary">▶ Play</button></div></div></div></div> <!></div>');function Sc(t,e){Je(e,!0);let r=O(null),a=O(!1),n=O(!1),i=O(!1);At(()=>{const m=e.artistId,P=e.artistName;x(r,null),x(n,!1),x(i,!1),He()&&(x(a,!0),Te.getArtist(m,P).then(D=>{e.artistId===m&&x(r,D,!0)}).catch(D=>o.showToast(String(D))).finally(()=>{e.artistId===m&&x(a,!1)}))});let l=Z(()=>s(r)?s(n)?s(r).tracks:s(r).tracks.slice(0,5):[]);var c=$c(),d=v(c),f=v(d);{var h=m=>{var P=fc();U(()=>{X(P,1,`hero-bg transition-opacity duration-300 ${s(i)?"opacity-100":"opacity-0"}`,"svelte-1iungmq"),it(P,"src",s(r).header_url)}),xt("load",P,()=>x(i,!0)),_(m,P)};F(f,m=>{s(r)?.header_url&&m(h)})}var p=u(f,2),g=v(p),b=u(g,2),w=u(v(b),2),L=N(w,!0),k=u(w,2);{var M=m=>{var P=hc(),D=N(P,!0);U(T=>I(D,T),[()=>_a(s(r).subscriber_count)]),_(m,P)};F(k,m=>{s(r)?.subscriber_count&&m(M)})}var A=u(k,2),z=N(A),q=u(d,2);{var C=m=>{var P=pc();_(m,P)},$=m=>{var P=kc(),D=se(P);{var T=ne=>{var Y=bc(),ue=u(v(Y),2);Ve(ue,23,()=>s(l),V=>V.id,(V,ie,W)=>{const ee=Z(()=>o.currentTrack?.id===s(ie).id);var E=_c();let J;var te=v(E);Ze(te,{class:"track-cover",get src(){return s(ie).cover_url},alt:""});var Le=u(te,2),he=v(Le),Oe=N(he,!0),re=u(he,2),ve=N(re),Ie=u(Le,2),Ce=N(Ie,!0),Ne=u(Ie,2);let je;var Ee=u(Ne,2);U((be,pe)=>{J=X(E,1,"track-row",null,J,{active:s(ee)}),I(Oe,s(ie).title),I(ve,`${s(ie).artist??""}${s(ie).album?` • ${s(ie).album}`:""}`),I(Ce,be),je=X(Ne,1,"icon-btn",null,je,{"fav-active":pe})},[()=>St(s(ie).duration_secs),()=>o.isFavorite(s(ie).id)]),y("click",E,()=>s(r)&&o.replaceQueueAndPlay(s(r).tracks,s(W),s(r).name)),y("keydown",E,be=>be.key==="Enter"&&s(r)&&o.replaceQueueAndPlay(s(r).tracks,s(W),s(r).name)),y("click",Ne,be=>{be.stopPropagation(),o.toggleFavorite(s(ie))}),y("click",Ee,be=>{be.stopPropagation(),o.enqueueTrack(s(ie))}),_(V,E)});var le=u(ue,2);{var ge=V=>{var ie=gc(),W=N(ie,!0);U(()=>I(W,s(n)?"▴ Show less":`▾ Show ${s(r).tracks.length-5} more`)),y("click",ie,()=>x(n,!s(n))),_(V,ie)};F(le,V=>{s(r).tracks.length>5&&V(ge)})}_(ne,Y)};F(D,ne=>{s(l).length>0&&ne(T)})}var j=u(D,2);{var R=ne=>{var Y=Ks(),ue=v(Y);Sr(ue,{title:"Albums",children:(le,ge)=>{var V=fe(),ie=se(V);Ve(ie,17,()=>s(r).albums,W=>W.id,(W,ee)=>{var E=mc(),J=v(E);Ze(J,{class:"media-cover",get src(){return s(ee).cover_url},get alt(){return s(ee).title}});var te=u(J,2),Le=N(te,!0),he=u(te,2),Oe=N(he,!0);U(()=>{I(Le,s(ee).title),I(Oe,s(ee).year??s(ee).artist)}),y("click",E,()=>o.openAlbum(s(ee).id,s(ee).title,s(ee).cover_url)),_(W,E)}),_(le,V)},$$slots:{default:!0}}),_(ne,Y)};F(j,ne=>{s(r).albums.length>0&&ne(R)})}var G=u(j,2);{var H=ne=>{var Y=Ks(),ue=v(Y);Sr(ue,{title:"Fans might also like",minCardWidth:144,children:(le,ge)=>{var V=fe(),ie=se(V);Ve(ie,17,()=>s(r).similar_artists,W=>W.id,(W,ee)=>{var E=yc(),J=v(E);Ze(J,{class:"artist-avatar",get src(){return s(ee).avatar_url},get alt(){return s(ee).name},fallbackIcon:"👤"});var te=u(J,2),Le=N(te,!0),he=u(te,2);{var Oe=re=>{var ve=xc(),Ie=N(ve,!0);U(Ce=>I(Ie,Ce),[()=>_a(s(ee).subscriber_count)]),_(re,ve)};F(he,re=>{s(ee).subscriber_count&&re(Oe)})}U(()=>I(Le,s(ee).name)),y("click",E,()=>o.openArtist(s(ee).id,s(ee).name)),_(W,E)}),_(le,V)},$$slots:{default:!0}}),_(ne,Y)};F(G,ne=>{s(r).similar_artists.length>0&&ne(H)})}var K=u(G,2);{var ce=ne=>{var Y=wc(),ue=v(Y),le=u(v(ue),2),ge=N(le,!0);U(()=>I(ge,s(r).description)),_(ne,Y)};F(K,ne=>{s(r).description&&ne(ce)})}_(m,P)};F(q,m=>{s(a)?m(C):s(r)&&m($,1)})}U(()=>{I(L,s(r)?.name??e.artistName),z.disabled=!s(r)?.tracks.length}),y("click",g,()=>o.navigateTab("explore")),y("click",z,()=>s(r)&&o.replaceQueueAndPlay(s(r).tracks,0,s(r).name)),_(t,c),Xe()}at(["click","keydown"]);var Tc=S('<p class="muted caption svelte-2o7gxz"> </p>'),Pc=S('<p class="muted empty-note svelte-2o7gxz">This playlist is empty. Add songs from your recent history or search results.</p>'),Ac=S('<div role="button" tabindex="0"><!> <div class="track-meta"><span class="track-title"> </span> <span class="track-sub"> </span></div> <span class="muted numeric dur svelte-2o7gxz"> </span> <button title="Favorite">♥</button> <button class="icon-btn" title="Remove from playlist">✕</button></div>'),Js=S('<div class="track-list"></div>'),Ec=S('<div class="detail-head svelte-2o7gxz"><button class="icon-btn back-btn svelte-2o7gxz" title="Back">‹</button> <div class="detail-badge svelte-2o7gxz"> </div> <div class="detail-meta svelte-2o7gxz"><span class="kicker svelte-2o7gxz">LOCAL PLAYLIST</span> <h1 class="page-title svelte-2o7gxz"> </h1> <!> <p class="muted caption svelte-2o7gxz"> </p> <div class="detail-actions svelte-2o7gxz"><button class="pill-btn primary">▶ Play all</button> <button class="pill-btn">Edit</button> <button class="pill-btn danger svelte-2o7gxz">Delete</button></div></div></div> <!>',1),Mc=S('<div class="playlist-card svelte-2o7gxz" role="button" tabindex="0"><span class="pl-badge svelte-2o7gxz"> </span> <div class="track-meta"><strong> </strong> <span class="muted small svelte-2o7gxz"> </span></div> <button class="icon-btn pl-play svelte-2o7gxz" title="Play playlist">▶</button></div>'),Ic=S('<button class="pill-btn">▶ Play all</button>'),Nc=S('<p class="muted">Songs you play will appear here.</p>'),zc=S('<button class="icon-btn" title="Add to playlist">☰</button>'),Lc=S('<button class="pill-btn small-pill svelte-2o7gxz"> </button>'),Cc=S('<div class="inline-picker svelte-2o7gxz"><span class="muted small svelte-2o7gxz">Add to:</span> <!></div>'),Rc=S('<div role="button" tabindex="0"><!> <div class="track-meta"><span class="track-title"> </span> <span class="track-sub"> </span></div> <span class="muted numeric dur svelte-2o7gxz"> </span> <button title="Favorite">♥</button> <!> <button class="icon-btn" title="Add to queue">+</button></div> <!>',1),qc=S('<div class="page-head svelte-2o7gxz"><div><h1 class="page-title svelte-2o7gxz">Collection</h1> <p class="muted caption svelte-2o7gxz">Your local playlists, recent history, and favorite songs</p></div> <button class="pill-btn primary">+ New playlist</button></div> <section class="section svelte-2o7gxz"><div class="section-head svelte-2o7gxz"><h2 class="section-title">Local playlists</h2></div> <div class="playlists-grid svelte-2o7gxz"><div class="playlist-card fav-card svelte-2o7gxz" role="button" tabindex="0"><span class="pl-badge fav-badge svelte-2o7gxz">♥</span> <div class="track-meta"><strong>Favorites</strong> <span class="muted small svelte-2o7gxz"> </span></div> <button class="icon-btn pl-play svelte-2o7gxz" title="Play favorites">▶</button></div> <!></div></section> <section class="section svelte-2o7gxz"><div class="section-head svelte-2o7gxz"><h2 class="section-title">Recently played</h2> <!></div> <!></section>',1),Oc=S('<button type="button"></button>'),jc=S('<button type="button"> </button>'),Fc=S('<div class="page-clamp"><!></div> <dialog class="gt-dialog"><form class="modal-form svelte-2o7gxz"><div class="modal-head svelte-2o7gxz"><h3 class="svelte-2o7gxz"> </h3> <button type="button" class="icon-btn">✕</button></div> <label class="field svelte-2o7gxz"><span>Name</span> <input type="text" placeholder="My playlist" required="" class="svelte-2o7gxz"/></label> <label class="field svelte-2o7gxz"><span>Description</span> <input type="text" placeholder="Optional description" class="svelte-2o7gxz"/></label> <div class="field svelte-2o7gxz"><span>Color</span> <div class="swatch-row svelte-2o7gxz"></div></div> <div class="field svelte-2o7gxz"><span>Icon</span> <div class="icon-grid svelte-2o7gxz"></div></div> <div class="modal-actions svelte-2o7gxz"><button type="button" class="pill-btn">Cancel</button> <button type="submit" class="pill-btn primary"> </button></div></form></dialog>',1);function Dc(t,e){Je(e,!0);const r=["#2190a4","#e01b24","#3584e4","#9141ac","#2ec27e","#e66100","#c061cb","#f5c211"],a=["♪","★","♥","⚡","🌙","☀","🔥","🎧","🎸","🎹","🚗","🏋"];let n=O(null),i=Z(()=>s(n)?o.userPlaylists.find(W=>W.id===s(n))??null:null),l=O(null),c=O(null),d=O(""),f=O(""),h=O(Ke(r[0])),p=O("♪"),g=O(null);function b(){x(c,null),x(d,""),x(f,""),x(h,r[0],!0),x(p,"♪"),s(l)?.showModal()}function w(W){x(c,W.id,!0),x(d,W.name,!0),x(f,W.description||"",!0),x(h,W.color_hex||r[0],!0),x(p,W.icon_name||"♪",!0),s(l)?.showModal()}function L(W){if(W.preventDefault(),!!s(d).trim()){if(s(c))o.updateUserPlaylist(s(c),s(d),s(f),s(h),s(p));else{const ee=o.createUserPlaylist(s(d),s(f),s(h),s(p));x(n,ee.id,!0)}s(l)?.close()}}function k(W){o.deleteUserPlaylist(W.id),s(n)===W.id&&x(n,null)}var M=Fc(),A=se(M),z=v(A);{var q=W=>{var ee=Ec(),E=se(ee),J=v(E),te=u(J,2),Le=N(te,!0),he=u(te,2),Oe=u(v(he),2),re=N(Oe,!0),ve=u(Oe,2);{var Ie=ae=>{var oe=Tc(),_e=N(oe,!0);U(()=>I(_e,s(i).description)),_(ae,oe)};F(ve,ae=>{s(i).description&&ae(Ie)})}var Ce=u(ve,2),Ne=N(Ce),je=u(Ce,2),Ee=v(je),be=u(Ee,2),pe=u(be,2),ze=u(E,2);{var Qe=ae=>{var oe=Pc();_(ae,oe)},me=ae=>{var oe=Js();Ve(oe,23,()=>s(i).tracks,_e=>_e.id,(_e,Se,We)=>{const Fe=Z(()=>o.currentTrack?.id===s(Se).id);var Ue=Ac();let B;var de=v(Ue);Ze(de,{class:"track-cover",get src(){return s(Se).cover_url},alt:""});var Re=u(de,2),Be=v(Re),_t=N(Be,!0),ct=u(Be,2),st=N(ct),Q=u(Re,2),ye=N(Q,!0),qe=u(Q,2);let ut;var dt=u(qe,2);U((vt,et)=>{B=X(Ue,1,"track-row",null,B,{active:s(Fe)}),I(_t,s(Se).title),I(st,`${s(Se).artist??""}${s(Se).album?` • ${s(Se).album}`:""}`),I(ye,vt),ut=X(qe,1,"icon-btn svelte-2o7gxz",null,ut,{"fav-active":et})},[()=>St(s(Se).duration_secs),()=>o.isFavorite(s(Se).id)]),y("click",Ue,()=>s(i)&&o.replaceQueueAndPlay(s(i).tracks,s(We),s(i).name)),y("keydown",Ue,vt=>vt.key==="Enter"&&s(i)&&o.replaceQueueAndPlay(s(i).tracks,s(We),s(i).name)),y("click",qe,vt=>{vt.stopPropagation(),o.toggleFavorite(s(Se))}),y("click",dt,vt=>{vt.stopPropagation(),s(i)&&o.removeTrackFromPlaylist(s(i).id,s(Se).id)}),_(_e,Ue)}),_(ae,oe)};F(ze,ae=>{s(i).tracks.length===0?ae(Qe):ae(me,-1)})}U(()=>{lt(te,`background: ${(s(i).color_hex||"var(--accent)")??""};`),I(Le,s(i).icon_name||"♪"),I(re,s(i).name),I(Ne,`${s(i).tracks.length??""} songs`),Ee.disabled=s(i).tracks.length===0}),y("click",J,()=>x(n,null)),y("click",Ee,()=>s(i)&&o.replaceQueueAndPlay(s(i).tracks,0,s(i).name)),y("click",be,()=>s(i)&&w(s(i))),y("click",pe,()=>s(i)&&k(s(i))),_(W,ee)},C=W=>{var ee=qc(),E=se(ee),J=u(v(E),2),te=u(E,2),Le=u(v(te),2),he=v(Le),Oe=u(v(he),2),re=u(v(Oe),2),ve=N(re),Ie=u(Oe,2),Ce=u(he,2);Ve(Ce,17,()=>o.userPlaylists,me=>me.id,(me,ae)=>{var oe=Mc(),_e=v(oe),Se=N(_e,!0),We=u(_e,2),Fe=v(We),Ue=N(Fe,!0),B=u(Fe,2),de=N(B),Re=u(We,2);U(()=>{lt(_e,`background: ${(s(ae).color_hex||"var(--accent)")??""};`),I(Se,s(ae).icon_name||"♪"),I(Ue,s(ae).name),I(de,`${s(ae).tracks.length??""} songs`),Re.disabled=s(ae).tracks.length===0}),y("click",oe,()=>x(n,s(ae).id,!0)),y("keydown",oe,Be=>Be.key==="Enter"&&x(n,s(ae).id,!0)),y("click",Re,Be=>{Be.stopPropagation(),s(ae).tracks.length&&o.replaceQueueAndPlay(s(ae).tracks,0,s(ae).name)}),_(me,oe)});var Ne=u(te,2),je=v(Ne),Ee=u(v(je),2);{var be=me=>{var ae=Ic();y("click",ae,()=>o.replaceQueueAndPlay(o.recentTracks,0,"Recent")),_(me,ae)};F(Ee,me=>{o.recentTracks.length>0&&me(be)})}var pe=u(je,2);{var ze=me=>{var ae=Nc();_(me,ae)},Qe=me=>{var ae=Js();Ve(ae,23,()=>o.recentTracks,oe=>oe.id,(oe,_e,Se)=>{const We=Z(()=>o.currentTrack?.id===s(_e).id);var Fe=Rc(),Ue=se(Fe);let B;var de=v(Ue);Ze(de,{class:"track-cover",get src(){return s(_e).cover_url},alt:""});var Re=u(de,2),Be=v(Re),_t=N(Be,!0),ct=u(Be,2),st=N(ct),Q=u(Re,2),ye=N(Q,!0),qe=u(Q,2);let ut;var dt=u(qe,2);{var vt=tt=>{var Gt=zc();y("click",Gt,cr=>{cr.stopPropagation(),x(g,s(g)?.id===s(_e).id?null:s(_e),!0)}),_(tt,Gt)};F(dt,tt=>{o.userPlaylists.length>0&&tt(vt)})}var et=u(dt,2),wt=u(Ue,2);{var Bt=tt=>{var Gt=Cc(),cr=u(v(Gt),2);Ve(cr,17,()=>o.userPlaylists,mt=>mt.id,(mt,Aa)=>{var Ea=Lc(),xi=N(Ea);U(()=>I(xi,`${(s(Aa).icon_name||"♪")??""} ${s(Aa).name??""}`)),y("click",Ea,()=>{o.addTrackToPlaylist(s(Aa).id,s(_e)),x(g,null)}),_(mt,Ea)}),_(tt,Gt)};F(wt,tt=>{s(g)?.id===s(_e).id&&tt(Bt)})}U((tt,Gt)=>{B=X(Ue,1,"track-row",null,B,{active:s(We)}),I(_t,s(_e).title),I(st,`${s(_e).artist??""}${s(_e).album?` • ${s(_e).album}`:""}`),I(ye,tt),ut=X(qe,1,"icon-btn svelte-2o7gxz",null,ut,{"fav-active":Gt})},[()=>St(s(_e).duration_secs),()=>o.isFavorite(s(_e).id)]),y("click",Ue,()=>o.replaceQueueAndPlay(o.recentTracks,s(Se),"Recent")),y("keydown",Ue,tt=>tt.key==="Enter"&&o.replaceQueueAndPlay(o.recentTracks,s(Se),"Recent")),y("click",qe,tt=>{tt.stopPropagation(),o.toggleFavorite(s(_e))}),y("click",et,tt=>{tt.stopPropagation(),o.enqueueTrack(s(_e))}),_(oe,Fe)}),_(me,ae)};F(pe,me=>{o.recentTracks.length===0?me(ze):me(Qe,-1)})}U(()=>{I(ve,`${o.favorites.length??""} songs`),Ie.disabled=o.favorites.length===0}),y("click",J,b),y("click",he,()=>o.favorites.length&&o.replaceQueueAndPlay(o.favorites,0,"Favorites")),y("keydown",he,me=>me.key==="Enter"&&o.favorites.length&&o.replaceQueueAndPlay(o.favorites,0,"Favorites")),_(W,ee)};F(z,W=>{s(i)?W(q):W(C,-1)})}var $=u(A,2),m=v($),P=v(m),D=v(P),T=N(D,!0),j=u(D,2),R=u(P,2),G=u(v(R),2),H=u(R,2),K=u(v(H),2),ce=u(H,2),ne=u(v(ce),2);Ve(ne,21,()=>r,_r,(W,ee)=>{var E=Oc();let J;U(()=>{J=X(E,1,"swatch svelte-2o7gxz",null,J,{selected:s(h)===s(ee)}),lt(E,`background: ${s(ee)??""};`),it(E,"aria-label",s(ee))}),y("click",E,()=>x(h,s(ee),!0)),_(W,E)});var Y=u(ce,2),ue=u(v(Y),2);Ve(ue,21,()=>a,_r,(W,ee)=>{var E=jc();let J;var te=N(E,!0);U(()=>{J=X(E,1,"icon-choice svelte-2o7gxz",null,J,{selected:s(p)===s(ee)}),I(te,s(ee))}),y("click",E,()=>x(p,s(ee),!0)),_(W,E)});var le=u(Y,2),ge=v(le),V=u(ge,2),ie=N(V,!0);br($,W=>x(l,W),()=>s(l)),U(()=>{I(T,s(c)?"Edit playlist":"New playlist"),I(ie,s(c)?"Save":"Create")}),xt("submit",m,L),y("click",j,()=>s(l)?.close()),fa(G,()=>s(d),W=>x(d,W)),fa(K,()=>s(f),W=>x(f,W)),y("click",ge,()=>s(l)?.close()),_(t,M),Xe()}at(["click","keydown"]);var Hc=S('<span class="muted"> </span>'),Vc=S('<section class="overview-col"><h3 class="col-title svelte-8hoooi">Top result</h3> <div class="top-result-card svelte-8hoooi" role="button" tabindex="0"><!> <h2 class="top-name svelte-8hoooi"> </h2> <div class="top-bottom svelte-8hoooi"><div class="top-meta svelte-8hoooi"><span class="type-pill svelte-8hoooi">Artist</span> <!></div> <button class="play-circle small-play svelte-8hoooi" title="Play">▶</button></div></div></section>'),Qc=S('<button class="see-all svelte-8hoooi">See all →</button>'),Xs=S('<div role="button" tabindex="0"><!> <div class="track-meta"><span class="track-title"> </span> <span class="track-sub"> </span></div> <span class="muted numeric dur svelte-8hoooi"> </span> <button title="Favorite">♥</button> <button class="icon-btn" title="Add to queue">+</button></div>'),Uc=S('<section class="overview-col songs-col"><div class="col-head svelte-8hoooi"><h3 class="col-title svelte-8hoooi">Songs</h3> <!></div> <div class="track-list"></div></section>'),Wc=S('<div class="overview-grid svelte-8hoooi"><!> <!></div>'),Bc=S('<span class="muted small svelte-8hoooi"> </span>'),Gc=S('<button class="artist-card svelte-8hoooi"><!> <strong> </strong> <!></button>'),Fa=S('<div class="section svelte-8hoooi"><!></div>'),Zs=S('<button class="media-card svelte-8hoooi"><!> <strong class="media-title svelte-8hoooi"> </strong> <span class="media-sub muted svelte-8hoooi"> </span></button>'),Yc=S('<section class="section svelte-8hoooi"><h2 class="section-title">Songs</h2> <div class="track-list"></div></section>'),Kc=S('<div class="filters svelte-8hoooi"><button class="back-explore svelte-8hoooi" title="Back to Explore">← Explore</button> <button>All</button> <button>Songs<!></button> <button>Albums<!></button> <button>Artists<!></button> <button>Playlists<!></button></div> <!> <!> <!> <!> <!>',1),Jc=S('<button class="mood-tile svelte-8hoooi"><span class="mood-dot svelte-8hoooi">♪</span> <span class="genre-name svelte-8hoooi"> </span> <span class="muted">›</span></button>'),Xc=S('<section class="section svelte-8hoooi"><h2 class="section-title">Moods &amp; moments</h2> <div class="genre-grid svelte-8hoooi"></div></section>'),Zc=S('<button class="genre-chip svelte-8hoooi"><span class="genre-bar svelte-8hoooi"></span> <span class="genre-name svelte-8hoooi"> </span> <span class="muted">›</span></button>'),eu=S('<section class="section svelte-8hoooi"><h2 class="section-title">Music genres</h2> <div class="genre-grid svelte-8hoooi"></div></section>'),tu=S('<h1 class="explore-title svelte-8hoooi">Explore</h1> <p class="muted caption svelte-8hoooi">Moods, moments, and music genres from YouTube Music</p> <!> <!>',1),ru=S('<div class="page-clamp"><!></div>');function au(t,e){Je(e,!0);let r=O("all");const a=["#2190a4","#e66100","#3584e4","#9141ac","#2ec27e","#e01b24","#c061cb","#f5c211","#00b4d8","#625df5"],n=["chill","christmas","commute","energize","feel good","focus","gaming","party","romance","sad","sleep","workout"];function i(k){let M=2166136261;for(let A=0;A<k.length;A++)M^=k.toLowerCase().charCodeAt(A),M=Math.imul(M,16777619);return a[Math.abs(M)%a.length]}let l=Z(()=>o.genres.filter(k=>n.some(M=>k.name.toLowerCase().includes(M)))),c=Z(()=>o.genres.filter(k=>!n.some(M=>k.name.toLowerCase().includes(M)))),d=Z(()=>o.searchResults),f=Z(()=>s(d)?.artists[0]??null),h=Z(()=>s(d)?.tracks.slice(0,4)??[]),p=Z(()=>s(r)==="all"&&!!s(f)&&(s(d)?.artists.length??0)===1);var g=ru(),b=v(g);{var w=k=>{var M=Kc(),A=se(M),z=v(A),q=u(z,2);let C;var $=u(q,2);let m;var P=u(v($));{var D=re=>{var ve=wr();U(()=>I(ve,` · ${s(d).tracks.length??""}`)),_(re,ve)};F(P,re=>{s(d).tracks.length&&re(D)})}var T=u($,2);let j;var R=u(v(T));{var G=re=>{var ve=wr();U(()=>I(ve,` · ${s(d).albums.length??""}`)),_(re,ve)};F(R,re=>{s(d).albums.length&&re(G)})}var H=u(T,2);let K;var ce=u(v(H));{var ne=re=>{var ve=wr();U(()=>I(ve,` · ${s(d).artists.length??""}`)),_(re,ve)};F(ce,re=>{s(d).artists.length&&re(ne)})}var Y=u(H,2);let ue;var le=u(v(Y));{var ge=re=>{var ve=wr();U(()=>I(ve,` · ${s(d).playlists.length??""}`)),_(re,ve)};F(le,re=>{s(d).playlists.length&&re(ge)})}var V=u(A,2);{var ie=re=>{var ve=Wc(),Ie=v(ve);{var Ce=Ee=>{var be=Vc(),pe=u(v(be),2),ze=v(pe);Ze(ze,{class:"top-avatar",get src(){return s(f).avatar_url},get alt(){return s(f).name},fallbackIcon:"👤"});var Qe=u(ze,2),me=N(Qe,!0),ae=u(Qe,2),oe=v(ae),_e=u(v(oe),2);{var Se=Fe=>{var Ue=Hc(),B=N(Ue,!0);U(de=>I(B,de),[()=>_a(s(f).subscriber_count)]),_(Fe,Ue)};F(_e,Fe=>{s(f).subscriber_count&&Fe(Se)})}var We=u(oe,2);U(()=>I(me,s(f).name)),y("click",pe,()=>o.openArtist(s(f).id,s(f).name)),y("keydown",pe,Fe=>Fe.key==="Enter"&&o.openArtist(s(f).id,s(f).name)),y("click",We,Fe=>{Fe.stopPropagation(),s(h)[0]&&o.replaceQueueAndPlay([s(h)[0]],0,`Search: ${o.searchQuery}`)}),_(Ee,be)};F(Ie,Ee=>{s(f)&&Ee(Ce)})}var Ne=u(Ie,2);{var je=Ee=>{var be=Uc(),pe=v(be),ze=u(v(pe),2);{var Qe=ae=>{var oe=Qc();y("click",oe,()=>x(r,"songs")),_(ae,oe)};F(ze,ae=>{s(d).tracks.length>4&&ae(Qe)})}var me=u(pe,2);Ve(me,21,()=>s(h),ae=>ae.id,(ae,oe)=>{const _e=Z(()=>o.currentTrack?.id===s(oe).id);var Se=Xs();let We;var Fe=v(Se);Ze(Fe,{class:"track-cover",get src(){return s(oe).cover_url},alt:""});var Ue=u(Fe,2),B=v(Ue),de=N(B,!0),Re=u(B,2),Be=N(Re),_t=u(Ue,2),ct=N(_t,!0),st=u(_t,2);let Q;var ye=u(st,2);U((qe,ut)=>{We=X(Se,1,"track-row",null,We,{active:s(_e)}),I(de,s(oe).title),I(Be,`${s(oe).artist??""}${s(oe).album?` • ${s(oe).album}`:""}`),I(ct,qe),Q=X(st,1,"icon-btn",null,Q,{"fav-active":ut})},[()=>St(s(oe).duration_secs),()=>o.isFavorite(s(oe).id)]),y("click",Se,()=>o.replaceQueueAndPlay([s(oe)],0,`Search: ${o.searchQuery}`)),y("keydown",Se,qe=>qe.key==="Enter"&&o.replaceQueueAndPlay([s(oe)],0,`Search: ${o.searchQuery}`)),y("click",st,qe=>{qe.stopPropagation(),o.toggleFavorite(s(oe))}),y("click",ye,qe=>{qe.stopPropagation(),o.enqueueTrack(s(oe))}),_(ae,Se)}),_(Ee,be)};F(Ne,Ee=>{s(h).length>0&&Ee(je)})}_(re,ve)};F(V,re=>{s(r)==="all"&&(s(f)||s(h).length>0)&&re(ie)})}var W=u(V,2);{var ee=re=>{var ve=Fa(),Ie=v(ve);Sr(Ie,{title:"Artists",minCardWidth:144,children:(Ce,Ne)=>{var je=fe(),Ee=se(je);Ve(Ee,17,()=>s(d).artists,be=>be.id,(be,pe)=>{var ze=Gc(),Qe=v(ze);Ze(Qe,{class:"artist-avatar",get src(){return s(pe).avatar_url},get alt(){return s(pe).name},fallbackIcon:"👤"});var me=u(Qe,2),ae=N(me,!0),oe=u(me,2);{var _e=Se=>{var We=Bc(),Fe=N(We,!0);U(Ue=>I(Fe,Ue),[()=>_a(s(pe).subscriber_count)]),_(Se,We)};F(oe,Se=>{s(pe).subscriber_count&&Se(_e)})}U(()=>I(ae,s(pe).name)),y("click",ze,()=>o.openArtist(s(pe).id,s(pe).name)),_(be,ze)}),_(Ce,je)},$$slots:{default:!0}}),_(re,ve)};F(W,re=>{(s(r)==="all"||s(r)==="artists")&&s(d).artists.length>0&&!s(p)&&re(ee)})}var E=u(W,2);{var J=re=>{var ve=Fa(),Ie=v(ve);Sr(Ie,{title:"Albums",children:(Ce,Ne)=>{var je=fe(),Ee=se(je);Ve(Ee,17,()=>s(d).albums,be=>be.id,(be,pe)=>{var ze=Zs(),Qe=v(ze);Ze(Qe,{class:"media-cover",get src(){return s(pe).cover_url},get alt(){return s(pe).title}});var me=u(Qe,2),ae=N(me,!0),oe=u(me,2),_e=N(oe,!0);U(()=>{I(ae,s(pe).title),I(_e,s(pe).artist)}),y("click",ze,()=>o.openAlbum(s(pe).id,s(pe).title,s(pe).cover_url)),_(be,ze)}),_(Ce,je)},$$slots:{default:!0}}),_(re,ve)};F(E,re=>{(s(r)==="all"||s(r)==="albums")&&s(d).albums.length>0&&re(J)})}var te=u(E,2);{var Le=re=>{var ve=Fa(),Ie=v(ve);Sr(Ie,{title:"Playlists",children:(Ce,Ne)=>{var je=fe(),Ee=se(je);Ve(Ee,17,()=>s(d).playlists,be=>be.id,(be,pe)=>{var ze=Zs(),Qe=v(ze);Ze(Qe,{class:"media-cover",get src(){return s(pe).cover_url},get alt(){return s(pe).name}});var me=u(Qe,2),ae=N(me,!0),oe=u(me,2),_e=N(oe,!0);U(()=>{I(ae,s(pe).name),I(_e,s(pe).subtitle)}),y("click",ze,()=>o.openAlbum(s(pe).id,s(pe).name,s(pe).cover_url,!0)),_(be,ze)}),_(Ce,je)},$$slots:{default:!0}}),_(re,ve)};F(te,re=>{(s(r)==="all"||s(r)==="playlists")&&s(d).playlists.length>0&&re(Le)})}var he=u(te,2);{var Oe=re=>{var ve=Yc(),Ie=u(v(ve),2);Ve(Ie,21,()=>s(d).tracks,Ce=>Ce.id,(Ce,Ne)=>{const je=Z(()=>o.currentTrack?.id===s(Ne).id);var Ee=Xs();let be;var pe=v(Ee);Ze(pe,{class:"track-cover",get src(){return s(Ne).cover_url},alt:""});var ze=u(pe,2),Qe=v(ze),me=N(Qe,!0),ae=u(Qe,2),oe=N(ae),_e=u(ze,2),Se=N(_e,!0),We=u(_e,2);let Fe;var Ue=u(We,2);U((B,de)=>{be=X(Ee,1,"track-row",null,be,{active:s(je)}),I(me,s(Ne).title),I(oe,`${s(Ne).artist??""}${s(Ne).album?` • ${s(Ne).album}`:""}`),I(Se,B),Fe=X(We,1,"icon-btn",null,Fe,{"fav-active":de})},[()=>St(s(Ne).duration_secs),()=>o.isFavorite(s(Ne).id)]),y("click",Ee,()=>o.replaceQueueAndPlay([s(Ne)],0,`Search: ${o.searchQuery}`)),y("keydown",Ee,B=>B.key==="Enter"&&o.replaceQueueAndPlay([s(Ne)],0,`Search: ${o.searchQuery}`)),y("click",We,B=>{B.stopPropagation(),o.toggleFavorite(s(Ne))}),y("click",Ue,B=>{B.stopPropagation(),o.enqueueTrack(s(Ne))}),_(Ce,Ee)}),_(re,ve)};F(he,re=>{s(r)==="songs"&&s(d).tracks.length>0&&re(Oe)})}U(()=>{C=X(q,1,"svelte-8hoooi",null,C,{active:s(r)==="all"}),m=X($,1,"svelte-8hoooi",null,m,{active:s(r)==="songs"}),j=X(T,1,"svelte-8hoooi",null,j,{active:s(r)==="albums"}),K=X(H,1,"svelte-8hoooi",null,K,{active:s(r)==="artists"}),ue=X(Y,1,"svelte-8hoooi",null,ue,{active:s(r)==="playlists"})}),y("click",z,()=>o.clearSearch()),y("click",q,()=>x(r,"all")),y("click",$,()=>x(r,"songs")),y("click",T,()=>x(r,"albums")),y("click",H,()=>x(r,"artists")),y("click",Y,()=>x(r,"playlists")),_(k,M)},L=k=>{var M=tu(),A=u(se(M),4);{var z=$=>{var m=Xc(),P=u(v(m),2);Ve(P,21,()=>s(l),D=>D.id,(D,T)=>{const j=Z(()=>i(s(T).name));var R=Jc(),G=v(R),H=u(G,2),K=N(H,!0);U(()=>{lt(R,`background: linear-gradient(135deg, ${s(j)??""}33, var(--surface) 68%); border-color: ${s(j)??""}44;`),lt(G,`background: ${s(j)??""}33; color: ${s(j)??""};`),I(K,s(T).name)}),y("click",R,()=>void o.runSearch(s(T).name)),_(D,R)}),_($,m)};F(A,$=>{s(l).length>0&&$(z)})}var q=u(A,2);{var C=$=>{var m=eu(),P=u(v(m),2);Ve(P,21,()=>s(c),D=>D.id,(D,T)=>{const j=Z(()=>i(s(T).name));var R=Zc(),G=v(R),H=u(G,2),K=N(H,!0);U(()=>{lt(R,`background: linear-gradient(90deg, ${s(j)??""}26, var(--surface) 62%);`),lt(G,`background: ${s(j)??""};`),I(K,s(T).name)}),y("click",R,()=>void o.runSearch(s(T).name)),_(D,R)}),_($,m)};F(q,$=>{s(c).length>0&&$(C)})}_(k,M)};F(b,k=>{s(d)?k(w):k(L,-1)})}_(t,g),Xe()}at(["click","keydown"]);var su=S('<button type="button" class="w-5 h-5 rounded-full inline-flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text)] hover:bg-[var(--surface-hover)] shrink-0 transition-colors" title="Clear search"><!></button>'),nu=S('<kbd class="hidden sm:inline-flex items-center px-1.5 py-0.5 rounded-md text-[10px] font-semibold text-[var(--text-muted)] bg-white/[0.05] border border-white/10 shrink-0">Ctrl K</kbd>'),iu=S('<button class="text-[var(--accent-bright)] hover:underline normal-case font-semibold">Clear</button>'),ou=S('<span class="text-[var(--text-muted)]"> </span><strong> </strong>',1),lu=S('<button><!> <span class="truncate"><!></span></button>'),cu=S('<div class="absolute left-0 right-0 top-[calc(100%+8px)] rounded-2xl bg-[var(--bg-elevated)]/98 backdrop-blur-xl border border-[var(--border-strong)] shadow-[0_18px_44px_rgba(0,0,0,0.5)] overflow-hidden py-1.5 z-50" role="listbox"><div class="flex items-center justify-between px-3.5 py-1.5 text-[11px] font-bold uppercase tracking-wider text-[var(--text-muted)]"><span> </span> <!></div> <!></div>'),uu=S('<span class="w-2 h-2 rounded-full bg-emerald-400 ring-2 ring-[var(--bg-elevated)] absolute top-1 right-1"></span>'),du=S('<header class="h-14 shrink-0 z-30 grid grid-cols-[1fr_minmax(220px,480px)_1fr] items-center gap-3 px-4 bg-[var(--bg-elevated)]/95 backdrop-blur-md border-b border-[var(--border)] select-none"><div class="justify-self-start flex items-center gap-2.5 min-w-0"><button title="Toggle player sidebar"><!></button> <button class="flex items-center gap-2 rounded-xl px-1.5 py-1 hover:bg-[var(--surface)] transition-colors" title="Go to Home"><img src="/gyrotube.svg" alt="" class="w-6 h-6 shrink-0 drop-shadow-[0_2px_8px_rgba(33,144,164,0.45)]"/> <span class="font-extrabold text-[15.5px] tracking-[-0.02em] hidden sm:inline">GyroTube</span> <span class="hidden md:inline-flex items-center px-2 py-0.5 rounded-full text-[10.5px] font-bold tracking-wide bg-[var(--accent-soft)] text-[var(--accent-bright)] border border-[var(--accent-border)]/40">Music</span></button></div> <div class="w-full relative"><div class="flex items-center gap-2.5 h-9 px-3.5 rounded-full bg-[var(--surface)] border border-[var(--border)] hover:border-[var(--border-strong)] focus-within:border-[var(--accent-border)] focus-within:bg-[var(--bg)] focus-within:ring-2 focus-within:ring-[var(--accent-soft)] transition-all"><!> <input type="text" class="w-full bg-transparent outline-none text-[13.5px] placeholder:text-[var(--text-muted)]" placeholder="Search songs, albums, artists…"/> <!></div> <!></div> <div class="justify-self-end flex items-center gap-1.5"><button title="SPIN Mode"><!> <!></button> <button title="Party Mode"><!></button> <span class="w-px h-5 bg-[var(--border-strong)] mx-1" aria-hidden="true"></span> <button title="Settings"><!></button></div></header>');function vu(t,e){Je(e,!0);let r=O(""),a=O(Ke([])),n=O(!1),i=O(-1),l,c=O(null),d=Z(()=>s(r).trim().length===0),f=Z(()=>s(d)?o.searchHistory:s(a)),h=Z(()=>o.spinStatus.local_active||o.spinStatus.remote_role!=="idle");At(()=>{!o.searchQuery&&s(r)&&document.activeElement!==s(c)&&x(r,"")}),Sa(()=>{const E=J=>{(J.ctrlKey||J.metaKey)&&J.key.toLowerCase()==="k"&&(J.preventDefault(),s(c)?.focus(),s(c)?.select())};return window.addEventListener("keydown",E),()=>window.removeEventListener("keydown",E)});function p(){x(i,-1),window.clearTimeout(l);const E=s(r).trim();if(!E){x(a,[],!0),o.clearSearch(),x(n,o.searchHistory.length>0);return}x(n,!0),l=window.setTimeout(async()=>{if(He())try{x(a,await Te.getSearchSuggestions(E),!0),x(n,s(a).length>0)}catch{x(a,[],!0)}},180)}function g(E){const J=E.trim();J&&(x(r,J,!0),x(n,!1),s(c)?.blur(),o.runSearch(J))}function b(){x(r,""),x(a,[],!0),x(i,-1),o.clearSearch(),x(n,o.searchHistory.length>0),s(c)?.focus()}function w(E){if(!s(n)||s(f).length===0){E.key==="Enter"?g(s(r)):E.key==="Escape"&&s(c)?.blur();return}E.key==="ArrowDown"?(E.preventDefault(),x(i,(s(i)+1)%s(f).length)):E.key==="ArrowUp"?(E.preventDefault(),x(i,(s(i)-1+s(f).length)%s(f).length)):E.key==="Enter"?(E.preventDefault(),g(s(i)>=0?s(f)[s(i)]:s(r))):E.key==="Escape"&&x(n,!1)}function L(E){const J=s(r).trim();return!s(d)&&J&&E.toLowerCase().startsWith(J.toLowerCase())?[E.slice(0,J.length),E.slice(J.length)]:["",E]}var k=du(),M=v(k),A=v(M),z=v(A);Ll(z,{size:18});var q=u(A,2),C=u(M,2),$=v(C),m=v($);Vs(m,{size:15,class:"text-[var(--text-muted)] shrink-0"});var P=u(m,2);br(P,E=>x(c,E),()=>s(c));var D=u(P,2);{var T=E=>{var J=su(),te=v(J);xs(te,{size:13}),y("mousedown",J,Le=>{Le.preventDefault(),b()}),_(E,J)},j=E=>{var J=nu();_(E,J)};F(D,E=>{s(r).length>0?E(T):E(j,-1)})}var R=u($,2);{var G=E=>{var J=cu(),te=v(J),Le=v(te),he=N(Le,!0),Oe=u(Le,2);{var re=Ie=>{var Ce=iu();y("mousedown",Ce,()=>o.clearSearchHistory()),_(Ie,Ce)};F(Oe,Ie=>{s(d)&&Ie(re)})}var ve=u(te,2);Ve(ve,17,()=>s(f),_r,(Ie,Ce,Ne)=>{const je=Z(()=>{const[ae,oe]=L(s(Ce));return{prefix:ae,rest:oe}});var Ee=lu(),be=v(Ee);Vs(be,{size:14,class:"text-[var(--text-muted)] shrink-0"});var pe=u(be,2),ze=v(pe);{var Qe=ae=>{var oe=ou(),_e=se(oe),Se=N(_e,!0),We=u(_e),Fe=N(We,!0);U(()=>{I(Se,s(je).prefix),I(Fe,s(je).rest)}),_(ae,oe)},me=ae=>{var oe=wr();U(()=>I(oe,s(Ce))),_(ae,oe)};F(ze,ae=>{s(je).prefix?ae(Qe):ae(me,-1)})}U(()=>X(Ee,1,`w-full flex items-center gap-2.5 px-3.5 py-2 text-left text-[13.5px] transition-colors ${Ne===s(i)?"bg-[var(--accent-soft)] text-[var(--accent-bright)]":"hover:bg-[var(--surface-hover)]"}`)),y("mousedown",Ee,()=>g(s(Ce))),_(Ie,Ee)}),U(()=>I(he,s(d)?"Recent searches":"Suggestions")),gt(3,J,()=>tr,()=>({y:-8,duration:170,easing:Mt})),_(E,J)};F(R,E=>{s(n)&&s(f).length>0&&E(G)})}var H=u(C,2),K=v(H);let ce;var ne=v(K);Dl(ne,{size:18});var Y=u(ne,2);{var ue=E=>{var J=uu();_(E,J)};F(Y,E=>{s(h)&&E(ue)})}var le=u(K,2);let ge;var V=v(le);_s(V,{size:18});var ie=u(le,4);let W;var ee=v(ie);ql(ee,{size:18}),U(()=>{X(A,1,`icon-btn shrink-0 ${o.sidebarOpen?"text-[var(--text)]":"text-[var(--text-muted)]"}`),ce=X(K,1,"icon-btn relative",null,ce,{active:o.spinOpen||s(h)}),ge=X(le,1,"icon-btn",null,ge,{active:o.partyMode}),W=X(ie,1,"icon-btn",null,W,{active:o.settingsOpen})}),y("click",A,()=>{o.sidebarOpen=!o.sidebarOpen,o.persistSession()}),y("click",q,()=>o.navigateTab("home")),xt("focus",P,()=>{s(f).length>0&&x(n,!0)}),xt("blur",P,()=>window.setTimeout(()=>x(n,!1),140)),y("input",P,p),y("keydown",P,w),fa(P,()=>s(r),E=>x(r,E)),y("click",K,()=>{o.spinOpen=!o.spinOpen,o.spinOpen&&o.refreshSpinStatus()}),y("click",le,()=>o.partyMode=!o.partyMode),y("click",ie,()=>o.settingsOpen=!0),_(t,k),Xe()}at(["click","input","keydown","mousedown"]);var fu=S('<section class="mt-6"><div class="flex flex-col sm:flex-row items-start sm:items-center gap-6 p-5 sm:px-6 rounded-[18px] border border-[var(--border)] shadow-[0_12px_32px_rgba(0,0,0,0.28)]" style="background: linear-gradient(135deg, rgba(var(--ambient-rgb), 0.24), var(--surface) 65%);"><!> <div class="min-w-0 flex flex-col gap-1.5"><span class="self-start text-[11px] font-extrabold tracking-[0.08em] px-2.5 py-0.5 rounded-full bg-[var(--accent-soft)] text-[var(--accent-bright)]">FEATURED</span> <h2 class="text-[24px] font-extrabold tracking-tight leading-tight"> </h2> <p class="text-[14px] text-[var(--text-muted)]"> </p> <div class="flex items-center gap-2.5 mt-1.5"><button class="pill-btn primary"><!> Play</button> <button class="pill-btn">Open album</button></div></div></div></section>');function hu(t,e){Je(e,!0);var r=fu(),a=v(r),n=v(a);Ze(n,{class:"hero-cover",get src(){return e.album.cover_url},get alt(){return e.album.title}});var i=u(n,2),l=u(v(i),2),c=N(l,!0),d=u(l,2),f=N(d,!0),h=u(d,2),p=v(h),g=v(p);Rr(g,{size:14,fill:"currentColor"});var b=u(p,2);U(()=>{I(c,e.album.title),I(f,e.album.artist)}),y("click",p,()=>o.openAlbum(e.album.id,e.album.title,e.album.cover_url)),y("click",b,()=>o.openAlbum(e.album.id,e.album.title,e.album.cover_url)),_(t,r),Xe()}at(["click"]);var pu=S('<button><!> <div class="track-meta"><span> </span> <span class="track-sub"> </span></div> <span aria-hidden="true"><!></span></button>'),_u=S('<section class="mt-6 flex flex-col gap-3"><h2 class="section-title">Listen again</h2> <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-2.5"></div></section>');function gu(t,e){Je(e,!0);var r=fe(),a=se(r);{var n=i=>{var l=_u(),c=u(v(l),2);Ve(c,23,()=>e.tracks,d=>d.id,(d,f,h)=>{const p=Z(()=>o.currentTrack?.id===s(f).id);var g=pu(),b=v(g);Ze(b,{class:"tile-cover",get src(){return s(f).cover_url},alt:""});var w=u(b,2),L=v(w),k=N(L,!0),M=u(L,2),A=N(M,!0),z=u(w,2),q=v(z);{var C=m=>{ha(m,{size:14})},$=m=>{Rr(m,{size:13,fill:"currentColor",class:"translate-x-[1px]"})};F(q,m=>{s(p)?m(C):m($,-1)})}U(()=>{X(g,1,`group flex items-center gap-3 p-2 pr-3 rounded-xl border text-left transition-all duration-150 ${s(p)?"bg-[var(--accent-soft)] border-[var(--accent-border)]":"bg-[var(--surface)] border-[var(--border)] hover:bg-[var(--surface-hover)]"}`),X(L,1,`track-title ${s(p)?"!text-[var(--accent-bright)] !font-bold":""}`),I(k,s(f).title),I(A,s(f).artist),X(z,1,`w-7 h-7 rounded-full inline-flex items-center justify-center shrink-0 transition-all duration-150 ${s(p)?"bg-[var(--accent-soft)] text-[var(--accent-bright)]":"text-[var(--text-muted)] group-hover:bg-[var(--accent)] group-hover:text-white group-hover:shadow-[0_4px_12px_rgba(33,144,164,0.42)]"}`)}),y("click",g,()=>o.replaceQueueAndPlay(e.tracks,s(h),"Listen again")),_(d,g)}),_(i,l)};F(a,i=>{e.tracks.length>0&&i(n)})}_(t,r),Xe()}at(["click"]);var bu=S('<section class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-2.5 mb-6"><button class="flex items-center gap-3 px-3.5 py-3 rounded-2xl border border-[var(--border)] hover:border-[var(--border-strong)] text-left transition-all duration-150 hover:-translate-y-0.5 active:scale-[0.98]" style="background: linear-gradient(135deg, rgba(224, 27, 36, 0.16), var(--surface) 70%);"><span class="w-9 h-9 rounded-xl inline-flex items-center justify-center bg-[var(--surface-hover)] shrink-0"><!></span> <span class="min-w-0 flex flex-col gap-0.5"><strong class="text-[14px] truncate">New releases</strong> <small class="text-[11.5px] text-[var(--text-muted)] truncate">Recent albums &amp; singles</small></span></button> <button class="flex items-center gap-3 px-3.5 py-3 rounded-2xl border border-[var(--border)] hover:border-[var(--border-strong)] text-left transition-all duration-150 hover:-translate-y-0.5 active:scale-[0.98]" style="background: linear-gradient(135deg, rgba(53, 132, 228, 0.16), var(--surface) 70%);"><span class="w-9 h-9 rounded-xl inline-flex items-center justify-center bg-[var(--surface-hover)] shrink-0"><!></span> <span class="min-w-0 flex flex-col gap-0.5"><strong class="text-[14px] truncate">Top charts</strong> <small class="text-[11.5px] text-[var(--text-muted)] truncate">Trending &amp; Top Music</small></span></button> <button class="flex items-center gap-3 px-3.5 py-3 rounded-2xl border border-[var(--border)] hover:border-[var(--border-strong)] text-left transition-all duration-150 hover:-translate-y-0.5 active:scale-[0.98]" style="background: linear-gradient(135deg, rgba(192, 97, 203, 0.16), var(--surface) 70%);"><span class="w-9 h-9 rounded-xl inline-flex items-center justify-center bg-[var(--surface-hover)] shrink-0"><!></span> <span class="min-w-0 flex flex-col gap-0.5"><strong class="text-[14px] truncate">Moods</strong> <small class="text-[11.5px] text-[var(--text-muted)] truncate">Explore genres &amp; vibes</small></span></button> <button class="flex items-center gap-3 px-3.5 py-3 rounded-2xl border border-[var(--border)] hover:border-[var(--border-strong)] text-left transition-all duration-150 hover:-translate-y-0.5 active:scale-[0.98]" style="background: linear-gradient(135deg, rgba(46, 194, 126, 0.16), var(--surface) 70%);"><span class="w-9 h-9 rounded-xl inline-flex items-center justify-center bg-[var(--surface-hover)] shrink-0"><!></span> <span class="min-w-0 flex flex-col gap-0.5"><strong class="text-[14px] truncate">Collection</strong> <small class="text-[11.5px] text-[var(--text-muted)] truncate">Your favorite songs</small></span></button></section>');function mu(t,e){Je(e,!1),di();var r=bu(),a=v(r),n=v(a),i=v(n);Pl(i,{size:18});var l=u(a,2),c=v(l),d=v(c);Fl(d,{size:18});var f=u(l,2),h=v(f),p=v(h);vi(p,{size:18});var g=u(f,2),b=v(g),w=v(b);Cr(w,{size:18}),y("click",a,()=>document.getElementById("new-releases")?.scrollIntoView({behavior:"smooth"})),y("click",l,()=>document.getElementById("trending-hits")?.scrollIntoView({behavior:"smooth"})),y("click",f,()=>{o.clearSearch(),o.navigateTab("explore")}),y("click",g,()=>o.navigateTab("collection")),_(t,r),Xe()}at(["click"]);var xu=S('<span class="w-4 h-4 rounded-full border-2 border-white/30 border-t-white animate-spin"></span>'),yu=S('<div class="group relative overflow-hidden rounded-2xl p-4 border border-white/10 hover:border-white/20 cursor-pointer transition-all duration-200 hover:-translate-y-0.5 flex flex-col justify-between min-h-[148px]" role="button" tabindex="0"><div class="flex items-center justify-between gap-2"><span class="px-2.5 py-0.5 rounded-full text-[11px] font-bold bg-black/30 backdrop-blur-sm">Station</span> <span class="opacity-85"><!></span></div> <div class="my-3"><h3 class="text-[18px] font-extrabold tracking-tight leading-snug"> </h3> <p class="text-[12px] text-[var(--text-muted)] mt-0.5 line-clamp-1">Curated playlists &amp; mixes for this style.</p></div> <div class="flex items-center justify-between gap-2"><span class="inline-flex items-center gap-1 text-[12px] font-bold group-hover:translate-x-0.5 transition-transform">Explore station <!></span> <button class="w-9 h-9 rounded-full inline-flex items-center justify-center bg-black/35 hover:bg-black/55 border border-white/10 text-white shadow-md transition-all active:scale-95"><!></button></div></div>'),wu=S('<section class="mt-6 flex flex-col gap-3"><div class="flex items-center justify-between gap-3"><h2 class="section-title">Stations &amp; genres</h2> <button class="text-[12.5px] font-semibold text-[var(--accent-bright)] hover:underline">See all →</button></div> <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3.5"></div></section>');function ku(t,e){Je(e,!0);const r=["#e05263","#2ec27e","#9141ac","#c061cb","#e66100","#2190a4"];let a=O(null);function n(h,p){return h.color_hex&&h.color_hex.startsWith("#")&&h.color_hex.length>=7?h.color_hex:r[p%r.length]}function i(h){const p=h.toLowerCase();return p.includes("chill")||p.includes("sleep")||p.includes("relax")?"moon":p.includes("commute")||p.includes("drive")||p.includes("travel")?"car":p.includes("energ")||p.includes("workout")||p.includes("party")?"zap":p.includes("feel")||p.includes("roman")||p.includes("love")?"heart":p.includes("focus")||p.includes("study")?"target":p.includes("christmas")||p.includes("holiday")||p.includes("mix")?"headphones":"music"}async function l(h){if(!(!He()||s(a))){x(a,h.id,!0);try{const[p,g]=await Te.getGenrePlaylists(h.id);if(g.length>0){const[b,w]=await Te.getPlaylistTracks(g[0].id);if(w.length>0){o.replaceQueueAndPlay(w,0,b||h.name);return}}await o.runSearch(h.name)}catch(p){o.showToast(String(p))}finally{x(a,null)}}}var c=fe(),d=se(c);{var f=h=>{var p=wu(),g=v(p),b=u(v(g),2),w=u(g,2);Ve(w,23,()=>e.stations,L=>L.id,(L,k,M)=>{const A=Z(()=>n(s(k),s(M))),z=Z(()=>i(s(k).name));var q=yu(),C=v(q),$=v(C),m=u($,2),P=v(m);{var D=E=>{zl(E,{size:16})},T=E=>{Tl(E,{size:16})},j=E=>{Hl(E,{size:16})},R=E=>{Cr(E,{size:16,fill:"currentColor"})},G=E=>{Ol(E,{size:16})},H=E=>{El(E,{size:16})},K=E=>{Ta(E,{size:16})};F(P,E=>{s(z)==="moon"?E(D):s(z)==="car"?E(T,1):s(z)==="zap"?E(j,2):s(z)==="heart"?E(R,3):s(z)==="target"?E(G,4):s(z)==="headphones"?E(H,5):E(K,-1)})}var ce=u(C,2),ne=v(ce),Y=N(ne,!0),ue=u(ce,2),le=v(ue),ge=u(v(le));Sl(ge,{size:13});var V=u(le,2),ie=v(V);{var W=E=>{var J=xu();_(E,J)},ee=E=>{Rr(E,{size:15,fill:"currentColor",class:"translate-x-[1px]"})};F(ie,E=>{s(a)===s(k).id?E(W):E(ee,-1)})}U(()=>{lt(q,`background: linear-gradient(145deg, ${s(A)??""}44 0%, ${s(A)??""}18 52%, var(--surface) 100%);`),lt($,`color: ${s(A)??""};`),lt(m,`color: ${s(A)??""};`),I(Y,s(k).name),lt(le,`color: ${s(A)??""};`),it(V,"title",`Play ${s(k).name??""}`),V.disabled=s(a)===s(k).id}),y("click",q,()=>void o.runSearch(s(k).name)),y("keydown",q,E=>E.key==="Enter"&&void o.runSearch(s(k).name)),y("click",V,E=>{E.stopPropagation(),l(s(k))}),_(L,q)}),y("click",b,()=>{o.clearSearch(),o.navigateTab("explore")}),_(h,p)};F(d,h=>{e.stations.length>0&&h(f)})}_(t,c),Xe()}at(["click","keydown"]);var $u=S('<div role="button" tabindex="0"><span class="w-6 text-center text-[12px] font-bold text-[var(--text-muted)] numeric shrink-0"><!></span> <!> <div class="track-meta"><span class="track-title"> </span> <span class="track-sub"> </span></div> <span class="text-[12px] text-[var(--text-muted)] numeric mr-1"> </span> <button title="Favorite"><!></button> <button class="icon-btn" title="Add to queue"><!></button></div>'),Su=S('<section class="mt-6 flex flex-col gap-3" id="trending-hits"><div class="flex items-center justify-between gap-3"><h2 class="section-title">Trending hits</h2> <div class="inline-flex gap-1 p-1 rounded-full bg-[var(--surface)] border border-[var(--border)]"><button>Trending</button> <button>Top Music</button></div></div> <div class="track-list"></div></section>'),Tu=S('<div class="py-12 text-center"><p class="text-[var(--text-muted)]">Connect Tauri runtime (`cargo tauri dev`) to load live YouTube Music feeds.</p></div>');function Pu(t,e){Je(e,!0);let r=O("trending"),a=Z(()=>s(r)==="top"?o.homeFeed?.top??[]:o.homeFeed?.trending??[]);var n=fe(),i=se(n);{var l=d=>{var f=Su(),h=v(f),p=u(v(h),2),g=v(p),b=u(g,2),w=u(h,2);Ve(w,23,()=>s(a),L=>L.id,(L,k,M)=>{const A=Z(()=>o.currentTrack?.id===s(k).id);var z=$u();let q;var C=v(z),$=v(C);{var m=V=>{ha(V,{size:15,class:"text-[var(--accent-bright)] mx-auto"})},P=V=>{var ie=wr();U(W=>I(ie,W),[()=>String(s(M)+1).padStart(2,"0")]),_(V,ie)};F($,V=>{s(A)?V(m):V(P,-1)})}var D=u(C,2);Ze(D,{class:"track-cover",get src(){return s(k).cover_url},alt:""});var T=u(D,2),j=v(T),R=N(j,!0),G=u(j,2),H=N(G,!0),K=u(T,2),ce=N(K,!0),ne=u(K,2);let Y;var ue=v(ne);{let V=Z(()=>o.isFavorite(s(k).id)?"currentColor":"none");Cr(ue,{size:16,get fill(){return s(V)}})}var le=u(ne,2),ge=v(le);pi(ge,{size:16}),U((V,ie)=>{q=X(z,1,"track-row cursor-pointer",null,q,{active:s(A)}),I(R,s(k).title),I(H,s(k).artist),I(ce,V),Y=X(ne,1,"icon-btn",null,Y,{"fav-active":ie})},[()=>St(s(k).duration_secs),()=>o.isFavorite(s(k).id)]),y("click",z,()=>o.replaceQueueAndPlay(s(a),s(M),"Trending hits")),y("keydown",z,V=>V.key==="Enter"&&o.replaceQueueAndPlay(s(a),s(M),"Trending hits")),y("click",ne,V=>{V.stopPropagation(),o.toggleFavorite(s(k))}),y("click",le,V=>{V.stopPropagation(),o.enqueueTrack(s(k))}),_(L,z)}),U(()=>{X(g,1,`px-3 py-1 rounded-full text-[12px] font-semibold transition-colors ${s(r)==="trending"?"bg-[var(--accent-soft)] text-[var(--accent-bright)]":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),X(b,1,`px-3 py-1 rounded-full text-[12px] font-semibold transition-colors ${s(r)==="top"?"bg-[var(--accent-soft)] text-[var(--accent-bright)]":"text-[var(--text-muted)] hover:text-[var(--text)]"}`)}),y("click",g,()=>x(r,"trending")),y("click",b,()=>x(r,"top")),_(d,f)},c=d=>{var f=Tu();_(d,f)};F(i,d=>{s(a).length>0?d(l):o.homeLoading||d(c,1)})}_(t,n),Xe()}at(["click","keydown"]);var Au=S('<span class="absolute top-2 left-2 px-2 py-0.5 rounded-md text-[11px] font-bold bg-black/65 text-white numeric"> </span>'),Eu=S('<button class="p-2.5 rounded-2xl bg-[var(--surface)] border border-[var(--border)] hover:bg-[var(--surface-hover)] hover:border-[var(--border-strong)] text-left flex flex-col gap-1.5 transition-colors"><div class="relative w-full"><!> <!></div> <strong class="text-[13.5px] truncate w-full"> </strong> <span class="text-[12px] text-[var(--text-muted)] truncate w-full"> </span></button>'),Mu=S('<div class="mt-6"><!></div>'),Iu=S('<div class="page-clamp"><div class="flex items-center justify-between mb-4"><div><h1 class="text-[28px] font-extrabold tracking-[-0.025em] leading-tight"> </h1> <p class="text-[13px] text-[var(--text-muted)] mt-0.5">Your music ready to play</p></div> <button class="icon-btn" title="Reload home"><!></button></div> <!> <!> <!> <!> <!> <!></div>');function Nu(t,e){Je(e,!0);let r=Z(()=>{const $=new Date().getHours();return $>=5&&$<12?"Good morning":$>=12&&$<19?"Good afternoon":"Good evening"}),a=Z(()=>o.recentTracks.slice(0,6)),n=Z(()=>o.homeFeed?.new_albums[0]??null),i=Z(()=>o.homeFeed?o.homeFeed.new_albums.slice(o.homeFeed.new_albums.length>1?1:0,16):[]),l=Z(()=>o.genres.slice(0,6));var c=Iu(),d=v(c),f=v(d),h=v(f),p=N(h,!0),g=u(f,2),b=v(g);{let $=Z(()=>o.homeLoading?"animate-spin":"");Rl(b,{size:16,get class(){return s($)}})}var w=u(d,2);mu(w,{});var L=u(w,2);gu(L,{get tracks(){return s(a)}});var k=u(L,2);{var M=$=>{hu($,{get album(){return s(n)}})};F(k,$=>{s(n)&&$(M)})}var A=u(k,2);{var z=$=>{var m=Mu(),P=v(m);Sr(P,{title:"New releases",id:"new-releases",children:(D,T)=>{var j=fe(),R=se(j);Ve(R,17,()=>s(i),G=>G.id,(G,H)=>{var K=Eu(),ce=v(K),ne=v(ce);Ze(ne,{class:"media-cover",get src(){return s(H).cover_url},get alt(){return s(H).title}});var Y=u(ne,2);{var ue=W=>{var ee=Au(),E=N(ee,!0);U(()=>I(E,s(H).year)),_(W,ee)};F(Y,W=>{s(H).year&&W(ue)})}var le=u(ce,2),ge=N(le,!0),V=u(le,2),ie=N(V,!0);U(()=>{I(ge,s(H).title),I(ie,s(H).artist)}),y("click",K,()=>o.openAlbum(s(H).id,s(H).title,s(H).cover_url)),_(G,K)}),_(D,j)},$$slots:{default:!0}}),_($,m)};F(A,$=>{s(i).length>0&&$(z)})}var q=u(A,2);Pu(q,{});var C=u(q,2);ku(C,{get stations(){return s(l)}}),U(()=>I(p,s(r))),y("click",g,()=>void o.loadHome(!0)),_(t,c),Xe()}at(["click"]);var zu=S("<div><!></div>"),en=S('<span class="w-1 h-1 rounded-full bg-[var(--accent-bright)] absolute bottom-1"></span>'),Lu=S('<button class="w-[340px] flex items-center gap-3 p-3 rounded-2xl bg-white/[0.05] hover:bg-white/[0.08] border border-white/10 text-left transition-colors"><!> <div class="min-w-0 flex-1"><span class="block text-[10px] font-extrabold uppercase tracking-[0.1em] text-[var(--text-muted)]">Up Next</span> <span class="block text-[13.5px] font-bold truncate"> </span> <span class="block text-[12px] text-[var(--text-muted)] truncate"> </span></div></button>'),Cu=S('<span class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[11px] font-bold bg-[var(--accent-soft)] text-[var(--accent-bright)]"><span class="w-1.5 h-1.5 rounded-full bg-[var(--accent-bright)]"></span> Synced</span>'),Ru=S('<span class="px-2.5 py-0.5 rounded-full text-[11px] font-semibold bg-white/8 text-[var(--text-muted)]">Static</span>'),qu=S("<button> </button>"),Ou=S('<button class="absolute bottom-6 left-1/2 -translate-x-1/2 inline-flex items-center gap-2 px-4 py-2 rounded-full text-[13px] font-bold bg-[var(--accent)] hover:bg-[var(--accent-bright)] text-white shadow-[0_10px_28px_rgba(0,0,0,0.5)] transition-all active:scale-95"><!> Sync</button>'),ju=S('<div class="relative flex-1 min-h-0 flex flex-col"><div role="region" aria-label="Lyrics" class="party-lyrics-scroll flex-1 min-h-0 overflow-y-auto py-[36vh] pr-8 pl-4 flex flex-col gap-8 svelte-1o5omgn"></div> <!></div>'),Fu=S('<div class="party-lyrics-scroll flex-1 min-h-0 overflow-y-auto py-12 pr-6 svelte-1o5omgn"><pre class="whitespace-pre-wrap font-sans text-[24px] font-bold leading-[1.6] tracking-[-0.02em] text-white/80"> </pre></div>'),Du=S('<div class="flex-1 flex flex-col items-center justify-center text-center gap-3 text-[var(--text-muted)]"><!> <p class="text-[18px] font-semibold">No lyrics available for this track</p></div>'),Hu=S('<div class="fixed inset-0 z-50 flex flex-col bg-[var(--bg)] text-[var(--text)] select-none overflow-hidden" style="background-image: radial-gradient(ellipse at 26% 36%, rgba(var(--ambient-rgb), 0.34) 0%, rgba(var(--ambient-rgb), 0.1) 42%, transparent 74%);"><header class="shrink-0 flex items-center justify-between px-7 py-5"><div class="flex items-center gap-3 min-w-0"><span class="inline-flex items-center gap-1.5 px-3.5 py-1.5 rounded-full text-[12px] font-bold tracking-wide bg-[var(--accent-soft)] text-[var(--accent-bright)] border border-[var(--accent-border)]/50"><!> Party Mode</span> <span class="text-[13.5px] font-medium text-[var(--text-muted)] truncate"> </span></div> <button class="inline-flex items-center gap-1.5 px-4 py-1.5 rounded-full text-[13px] font-semibold bg-white/8 hover:bg-white/14 border border-white/10 transition-colors"><!> Exit</button></header> <div class="flex-1 min-h-0 grid grid-cols-1 lg:grid-cols-[440px_minmax(0,1fr)] gap-10 px-10 pb-8 max-w-[1440px] w-full mx-auto"><div class="flex flex-col items-center justify-center gap-5"><!> <div class="w-[340px] text-center space-y-1"><h1 class="text-[28px] font-black tracking-[-0.03em] leading-tight line-clamp-2"> </h1> <p class="text-[16px] font-medium text-[var(--text-muted)] truncate"> </p></div> <div class="w-[340px] space-y-2.5"><div class="h-1.5 w-full rounded-full bg-white/15 cursor-pointer overflow-hidden group" role="slider" tabindex="0" aria-label="Seek"><div class="h-full rounded-full bg-[var(--accent-bright)] transition-[width] duration-150 ease-linear"></div></div> <div class="flex items-center justify-between text-[12.5px] font-medium text-[var(--text-muted)] numeric"><span> </span> <button><!></button> <span> </span></div></div> <div class="flex items-center justify-center gap-4"><button title="Shuffle"><!> <!></button> <button class="icon-btn !w-11 !h-11" title="Previous"><!></button> <button class="play-circle !w-[62px] !h-[62px]" title="Play / Pause"><!></button> <button class="icon-btn !w-11 !h-11" title="Next"><!></button> <button title="Repeat"><!> <!></button></div> <!></div> <div class="min-h-0 flex flex-col"><div class="shrink-0 flex items-center gap-2.5 pb-2"><span class="text-[13px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)]">Lyrics</span> <!></div> <!></div></div></div>');function Vu(t,e){Je(e,!0);let r=Ke({}),a=O(!1),n=O(null),i=Z(()=>o.currentTrack?o.isFavorite(o.currentTrack.id):!1),l=Z(()=>o.durationSecs>0?Math.max(0,Math.min(100,o.positionSecs/o.durationSecs*100)):0);At(()=>{const B=o.currentTrack?.id??null;B!==s(n)&&(x(n,B,!0),x(a,!1))}),At(()=>{const B=o.activeLyricIndex;B!=null&&o.partyMode&&!s(a)&&r[B]?.scrollIntoView({behavior:"smooth",block:"center"})});function c(){x(a,!0)}function d(){x(a,!1);const B=o.activeLyricIndex;B!=null&&r[B]?.scrollIntoView({behavior:"smooth",block:"center"})}function f(B){if(o.activeLyricIndex==null)return .6;if(B===o.activeLyricIndex)return 1;const de=Math.abs(B-o.activeLyricIndex);return de===1?.36:de===2?.2:.11}function h(B){return o.activeLyricIndex==null?!1:Math.abs(B-o.activeLyricIndex)>=2}function p(B){if(!o.currentTrack||o.durationSecs<=0)return;const de=B.currentTarget.getBoundingClientRect(),Re=Math.max(0,Math.min(1,(B.clientX-de.left)/de.width));o.seek(Re*o.durationSecs)}var g=Hu(),b=v(g),w=v(b),L=v(w),k=v(L);_s(k,{size:14});var M=u(L,2),A=N(M,!0),z=u(w,2),q=v(z);xs(q,{size:15});var C=u(b,2),$=v(C),m=v($);ps(m,()=>o.currentTrack?.id??"none",B=>{var de=zu(),Re=v(de);{let Be=Z(()=>o.currentTrack?.cover_large_url),_t=Z(()=>o.currentTrack?.cover_url),ct=Z(()=>o.currentTrack?.title??"");Ze(Re,{class:"party-cover !w-[292px] !h-[292px] !rounded-[22px] border border-white/12 shadow-[0_26px_64px_rgba(0,0,0,0.58)]",get src(){return s(Be)},get fallbackSrc(){return s(_t)},get alt(){return s(ct)},eager:!0})}gt(1,de,()=>pa,()=>({start:.94,duration:280,easing:Mt})),_(B,de)});var P=u(m,2),D=v(P),T=N(D,!0),j=u(D,2),R=N(j,!0),G=u(P,2),H=v(G);it(H,"aria-valuemin",0);var K=N(H),ce=u(H,2),ne=v(ce),Y=N(ne,!0),ue=u(ne,2),le=v(ue);{let B=Z(()=>s(i)?"currentColor":"none");Cr(le,{size:18,get fill(){return s(B)}})}var ge=u(ue,2),V=N(ge,!0),ie=u(G,2),W=v(ie),ee=v(W);gi(ee,{size:18});var E=u(ee,2);{var J=B=>{var de=en();_(B,de)};F(E,B=>{o.shuffle&&B(J)})}var te=u(W,2),Le=v(te);bs(Le,{size:22,fill:"currentColor"});var he=u(te,2),Oe=v(he);{var re=B=>{gs(B,{size:24,fill:"currentColor"})},ve=B=>{Rr(B,{size:24,fill:"currentColor",class:"translate-x-[1.5px]"})};F(Oe,B=>{o.isPlaying?B(re):B(ve,-1)})}var Ie=u(he,2),Ce=v(Ie);ms(Ce,{size:22,fill:"currentColor"});var Ne=u(Ie,2),je=v(Ne);_i(je,{size:18});var Ee=u(je,2);{var be=B=>{var de=en();_(B,de)};F(Ee,B=>{o.repeat&&B(be)})}var pe=u(ie,2);{var ze=B=>{var de=Lu(),Re=v(de);Ze(Re,{class:"up-next-cover !w-11 !h-11 !rounded-lg",get src(){return o.nextTrack.cover_url},alt:""});var Be=u(Re,2),_t=u(v(Be),2),ct=N(_t,!0),st=u(_t,2),Q=N(st,!0);U(()=>{I(ct,o.nextTrack.title),I(Q,o.nextTrack.artist)}),y("click",de,()=>void o.next()),_(B,de)};F(pe,B=>{o.nextTrack&&B(ze)})}var Qe=u($,2),me=v(Qe),ae=u(v(me),2);{var oe=B=>{var de=Cu();_(B,de)},_e=B=>{var de=Ru();_(B,de)};F(ae,B=>{o.lyrics.type==="Synced"?B(oe):o.lyrics.type==="Plain"&&B(_e,1)})}var Se=u(me,2);{var We=B=>{var de=ju(),Re=v(de);Ve(Re,21,()=>o.lyrics.lines,_r,(ct,st,Q)=>{const ye=Z(()=>o.activeLyricIndex===Q),qe=Z(()=>Q>0?o.lyrics.lines[Q-1]:null),ut=Z(()=>s(qe)!=null&&s(st).time_secs-s(qe).time_secs>=5.5);var dt=qu(),vt=N(dt,!0);br(dt,(et,wt)=>r[wt]=et,et=>r?.[et],()=>[Q]),U((et,wt)=>{X(dt,1,`party-line text-left origin-left ${s(ye)?"active-line":""} ${et??""} ${s(ut)?"mt-5":""}`,"svelte-1o5omgn"),lt(dt,`opacity: ${wt??""};`),I(vt,s(st).text)},[()=>h(Q)?"far-line":"",()=>f(Q)]),y("click",dt,()=>{x(a,!1),o.seek(s(st).time_secs)}),_(ct,dt)});var Be=u(Re,2);{var _t=ct=>{var st=Ou(),Q=v(st);hi(Q,{size:15}),y("click",st,d),gt(3,st,()=>pa,()=>({start:.88,duration:180,easing:Mt})),_(ct,st)};F(Be,ct=>{s(a)&&o.activeLyricIndex!=null&&ct(_t)})}xt("wheel",Re,c),y("touchmove",Re,c),_(B,de)},Fe=B=>{var de=Fu(),Re=v(de),Be=N(Re,!0);U(()=>I(Be,o.lyrics.text)),_(B,de)},Ue=B=>{var de=Du(),Re=v(de);Ta(Re,{size:40,class:"opacity-30"}),_(B,de)};F(Se,B=>{o.lyrics.type==="Synced"?B(We):o.lyrics.type==="Plain"?B(Fe,1):B(Ue,-1)})}U((B,de,Re,Be)=>{I(A,o.sourceName),I(T,o.currentTrack?.title??"Nothing playing"),I(R,o.currentTrack?.artist??""),it(H,"aria-valuemax",B),it(H,"aria-valuenow",de),lt(K,`width: ${s(l)??""}%`),I(Y,Re),X(ue,1,`icon-btn !w-9 !h-9 transition-transform active:scale-90 ${s(i)?"text-[var(--coral)]":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),ue.disabled=!o.currentTrack,it(ue,"title",s(i)?"Remove from favorites":"Add to favorites"),I(V,Be),X(W,1,`icon-btn !w-10 !h-10 relative ${o.shuffle?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),te.disabled=!o.currentTrack,he.disabled=!o.currentTrack,Ie.disabled=!o.currentTrack,X(Ne,1,`icon-btn !w-10 !h-10 relative ${o.repeat?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`)},[()=>Math.round(o.durationSecs),()=>Math.round(o.positionSecs),()=>St(o.positionSecs),()=>St(o.durationSecs)]),y("click",z,()=>o.partyMode=!1),y("click",H,p),y("keydown",H,B=>{B.key==="ArrowRight"?o.seek(o.positionSecs+5):B.key==="ArrowLeft"&&o.seek(Math.max(0,o.positionSecs-5))}),y("click",ue,()=>o.currentTrack&&o.toggleFavorite(o.currentTrack)),y("click",W,()=>o.toggleShuffle()),y("click",te,()=>void o.prev()),y("click",he,()=>void o.togglePlay()),y("click",Ie,()=>void o.next()),y("click",Ne,()=>o.toggleRepeat()),_(t,g),Xe()}at(["click","keydown","touchmove"]);var Qu=S('<dialog class="gt-dialog"><div class="dlg-head svelte-qv6s6m"><h2 class="svelte-qv6s6m">Settings</h2> <button class="icon-btn">✕</button></div> <div class="dlg-body svelte-qv6s6m"><section class="group svelte-qv6s6m"><h3 class="svelte-qv6s6m">Appearance</h3> <div class="card svelte-qv6s6m"><div class="row svelte-qv6s6m"><span>Theme</span> <select class="svelte-qv6s6m"><option>System</option><option>Dark</option><option>Light</option></select></div></div></section> <section class="group svelte-qv6s6m"><h3 class="svelte-qv6s6m">Playback</h3> <div class="card svelte-qv6s6m"><div class="row svelte-qv6s6m"><div class="svelte-qv6s6m"><strong>Crossfade</strong> <small class="muted svelte-qv6s6m"> </small></div> <input type="range" min="0" max="10" step="1"/></div> <label class="row svelte-qv6s6m"><div class="svelte-qv6s6m"><strong>Autoplay</strong> <small class="muted svelte-qv6s6m">Queue similar songs when the queue ends</small></div> <input type="checkbox"/></label> <label class="row svelte-qv6s6m"><div class="svelte-qv6s6m"><strong>Background playback</strong> <small class="muted svelte-qv6s6m">Keep playing music when closing the window</small></div> <input type="checkbox"/></label></div></section> <section class="group svelte-qv6s6m"><h3 class="svelte-qv6s6m">Storage</h3> <div class="card svelte-qv6s6m"><div class="row svelte-qv6s6m"><div class="svelte-qv6s6m"><strong>Audio cache</strong> <small class="muted numeric svelte-qv6s6m"> </small> <div class="cache-bar svelte-qv6s6m"><div class="cache-fill svelte-qv6s6m"></div></div></div> <button class="pill-btn danger svelte-qv6s6m">Clear</button></div> <div class="row svelte-qv6s6m"><div class="svelte-qv6s6m"><strong>Search history</strong> <small class="muted svelte-qv6s6m"> </small></div> <button class="pill-btn">Clear</button></div></div></section></div></dialog>');function Uu(t,e){Je(e,!0);let r=O(null),a=O(0);At(()=>{const te=s(r);te&&(o.settingsOpen&&!te.open?(te.showModal(),He()&&Te.getCacheBytes().then(Le=>x(a,Le,!0)).catch(()=>{})):!o.settingsOpen&&te.open&&te.close())});let n=Z(()=>(s(a)/(1024*1024)).toFixed(1)),i=Z(()=>Math.min(100,s(a)/(1024*1024*1024)*100));async function l(){He()&&(await Te.clearAudioCache(),x(a,0),o.showToast("Audio cache cleared"))}var c=Qu(),d=v(c),f=u(v(d),2),h=u(d,2),p=v(h),g=u(v(p),2),b=v(g),w=u(v(b),2),L=v(w);L.value=L.__value="system";var k=u(L);k.value=k.__value="dark";var M=u(k);M.value=M.__value="light";var A;ni(w);var z=u(p,2),q=u(v(z),2),C=v(q),$=v(C),m=u(v($),2),P=N(m,!0),D=u($,2),T=u(C,2),j=u(v(T),2),R=u(T,2),G=u(v(R),2),H=u(z,2),K=u(v(H),2),ce=v(K),ne=v(ce),Y=u(v(ne),2),ue=N(Y),le=u(Y,2),ge=N(le),V=u(ne,2),ie=u(ce,2),W=v(ie),ee=u(v(W),2),E=N(ee),J=u(W,2);br(c,te=>x(r,te),()=>s(r)),U(()=>{A!==(A=o.themeMode)&&(w.value=(w.__value=A)??"",va(w,A)),I(P,o.crossfadeSecs===0?"Disabled (0 s)":`${o.crossfadeSecs} s`),es(D,o.crossfadeSecs),Wr(j,o.autoplayRelated),Wr(G,o.backgroundPlayback),I(ue,`${s(n)??""} MB / 1024 MB cached`),lt(ge,`width: ${s(i)??""}%`),V.disabled=s(a)===0,I(E,`${o.searchHistory.length??""} saved searches`),J.disabled=o.searchHistory.length===0}),xt("close",c,()=>o.settingsOpen=!1),y("click",c,te=>{te.target===s(r)&&(o.settingsOpen=!1)}),y("click",f,()=>o.settingsOpen=!1),y("change",w,te=>o.setThemeMode(te.currentTarget.value)),y("input",D,te=>o.setCrossfadeSecs(Number(te.currentTarget.value))),y("change",j,te=>o.setAutoplay(te.currentTarget.checked)),y("change",G,te=>o.setBackgroundPlayback(te.currentTarget.checked)),y("click",V,()=>void l()),y("click",J,()=>o.clearSearchHistory()),_(t,c),Xe()}at(["click","change","input"]);var Wu=S('<p class="text-[12px] text-[var(--text-muted)] truncate"> </p>'),Bu=S('<span class="shrink-0 inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full text-[10.5px] font-bold bg-[var(--accent-soft)] text-[var(--accent-bright)]"><span class="w-1.5 h-1.5 rounded-full bg-[var(--accent-bright)]"></span> Synced</span>'),Gu=S('<span class="shrink-0 px-2.5 py-0.5 rounded-full text-[10.5px] font-semibold bg-[var(--surface)] text-[var(--text-muted)]">Static</span>'),Yu=S("<button> </button>"),Ku=S('<button class="absolute bottom-3 left-1/2 -translate-x-1/2 inline-flex items-center gap-1.5 px-3.5 py-1.5 rounded-full text-[12px] font-bold bg-[var(--accent)] hover:bg-[var(--accent-bright)] text-white shadow-[0_8px_24px_rgba(0,0,0,0.45)] transition-all active:scale-95"><!> Sync</button>'),Ju=S('<div class="relative flex-1 min-h-0 flex flex-col"><div role="region" aria-label="Lyrics" class="lyrics-fade flex-1 min-h-0 overflow-y-auto overflow-x-hidden px-2 pt-28 pb-36 flex flex-col gap-4 svelte-cw8tlx"></div> <!></div>'),Xu=S('<div class="lyrics-fade flex-1 min-h-0 overflow-y-auto py-8 svelte-cw8tlx"><pre class="whitespace-pre-wrap font-sans text-[14.5px] leading-relaxed text-[var(--text)]/85"> </pre></div>'),Zu=S('<div class="flex-1 flex flex-col items-center justify-center text-center gap-2 py-12 text-[var(--text-muted)]"><!> <p class="text-[13px]"> </p></div>'),ed=S('<div class="flex flex-col gap-3 pt-1 flex-1 min-h-0"><div class="flex items-start justify-between gap-2 pb-2 border-b border-[var(--border)]"><div class="min-w-0"><h3 class="text-[14.5px] font-bold truncate"> </h3> <!></div> <!></div> <!></div>');function td(t,e){Je(e,!0);let r=Ke({}),a=O(!1),n=O(null);At(()=>{const m=o.currentTrack?.id??null;m!==s(n)&&(x(n,m,!0),x(a,!1))}),At(()=>{const m=o.activeLyricIndex;m!=null&&!s(a)&&r[m]?.scrollIntoView({behavior:"smooth",block:"center"})});function i(){x(a,!0)}function l(){x(a,!1);const m=o.activeLyricIndex;m!=null&&r[m]?.scrollIntoView({behavior:"smooth",block:"center"})}function c(m){if(o.activeLyricIndex==null)return .65;if(m===o.activeLyricIndex)return 1;const P=Math.abs(m-o.activeLyricIndex);return P===1?.42:P===2?.24:.14}function d(m){return o.activeLyricIndex==null?!1:Math.abs(m-o.activeLyricIndex)>=2}var f=ed(),h=v(f),p=v(h),g=v(p),b=N(g,!0),w=u(g,2);{var L=m=>{var P=Wu(),D=N(P,!0);U(()=>I(D,o.currentTrack.artist)),_(m,P)};F(w,m=>{o.currentTrack&&m(L)})}var k=u(p,2);{var M=m=>{var P=Bu();_(m,P)},A=m=>{var P=Gu();_(m,P)};F(k,m=>{o.lyrics.type==="Synced"?m(M):o.lyrics.type==="Plain"&&m(A,1)})}var z=u(h,2);{var q=m=>{var P=Ju(),D=v(P);Ve(D,21,()=>o.lyrics.lines,_r,(R,G,H)=>{const K=Z(()=>o.activeLyricIndex===H),ce=Z(()=>H>0?o.lyrics.lines[H-1]:null),ne=Z(()=>s(ce)!=null&&s(G).time_secs-s(ce).time_secs>=5.5);var Y=Yu(),ue=N(Y,!0);br(Y,(le,ge)=>r[ge]=le,le=>r?.[le],()=>[H]),U((le,ge)=>{X(Y,1,`sb-lyric-line text-left origin-left ${s(K)?"active-line":""} ${le??""} ${s(ne)?"mt-3":""}`,"svelte-cw8tlx"),lt(Y,`opacity: ${ge??""};`),I(ue,s(G).text)},[()=>d(H)?"far-line":"",()=>c(H)]),y("click",Y,()=>{x(a,!1),o.seek(s(G).time_secs)}),_(R,Y)});var T=u(D,2);{var j=R=>{var G=Ku(),H=v(G);hi(H,{size:14}),y("click",G,l),gt(3,G,()=>pa,()=>({start:.88,duration:170,easing:Mt})),_(R,G)};F(T,R=>{s(a)&&o.activeLyricIndex!=null&&R(j)})}xt("wheel",D,i),y("touchmove",D,i),_(m,P)},C=m=>{var P=Xu(),D=v(P),T=N(D,!0);U(()=>I(T,o.lyrics.text)),_(m,P)},$=m=>{var P=Zu(),D=v(P);Ta(D,{size:28,class:"opacity-40"});var T=u(D,2),j=N(T,!0);U(()=>I(j,o.currentTrack?"No lyrics available for this track.":"Play a song to view its lyrics.")),_(m,P)};F(z,m=>{o.lyrics.type==="Synced"?m(q):o.lyrics.type==="Plain"?m(C,1):m($,-1)})}U(()=>I(b,o.currentTrack?.title??"Lyrics")),gt(1,f,()=>Pa,()=>({duration:180})),_(t,f),Xe()}at(["touchmove","click"]);var rd=S("<div><!></div>"),ad=S('<p class="text-[12px] text-[var(--text-muted)]/75 truncate"> </p>'),sd=S('<span class="w-4 h-4 rounded-full border-2 border-current/30 border-t-current animate-spin"></span>'),nd=S('<span class="w-1.5 h-1.5 rounded-full bg-[var(--accent-bright)] absolute top-1 right-1"></span>'),id=S('<button type="button" aria-label="Close popover" class="fixed inset-0 z-20 cursor-default bg-transparent"></button>'),od=S('<div class="absolute bottom-12 left-1/2 -translate-x-1/2 w-[272px] p-3.5 rounded-2xl bg-[var(--bg-elevated)]/95 backdrop-blur-md border border-[var(--border-strong)] shadow-[0_16px_40px_rgba(0,0,0,0.5)] z-30 space-y-2.5"><div class="flex items-center justify-between text-[12px]"><span class="font-semibold text-[var(--text-muted)]">Volume</span> <span class="font-bold text-[var(--accent-bright)] numeric"> </span></div> <div class="flex items-center gap-3"><button><!></button> <input class="sidebar-seek flex-1 min-w-0 svelte-12h25u0" type="range" min="0" max="1" step="0.01" aria-label="Volume"/></div></div>'),ld=S('<span class="font-bold text-[var(--accent-bright)] numeric"> </span>'),cd=S('<button class="py-1.5 rounded-xl text-[12px] font-semibold bg-[var(--surface)] text-[var(--text-muted)] hover:bg-[var(--accent-soft)] hover:text-[var(--accent-bright)] transition-colors"> </button>'),ud=S('<div class="absolute bottom-12 left-1/2 -translate-x-1/2 w-[272px] p-3.5 rounded-2xl bg-[var(--bg-elevated)]/95 backdrop-blur-md border border-[var(--border-strong)] shadow-[0_16px_40px_rgba(0,0,0,0.5)] z-30 space-y-2.5"><div class="flex items-center justify-between text-[12px]"><span class="font-semibold text-[var(--text-muted)]">Sleep timer</span> <!></div> <div class="grid grid-cols-5 gap-1.5"></div></div>'),tn=S('<span class="w-1 h-1 rounded-full bg-[var(--accent-bright)] absolute bottom-0.5"></span>'),dd=S('<span class="w-5 h-5 rounded-full border-2 border-white/30 border-t-white animate-spin"></span>'),vd=S('<div class="flex-1 flex flex-col items-center justify-between py-4 gap-5"><div class="flex-1 flex items-center justify-center w-full py-2"><!></div> <div class="w-full flex flex-col items-center gap-4"><div class="w-full text-center px-1 space-y-1"><h2 class="text-[19px] font-extrabold leading-tight tracking-tight line-clamp-2"> </h2> <p class="text-[14px] font-medium text-[var(--text-muted)] truncate"> </p> <!></div> <div class="relative w-full flex items-center justify-center gap-5 py-0.5"><button><!></button> <button><!></button> <button title="Start radio from this song"><!></button> <button><!> <!></button> <button title="Party Mode"><!></button> <!> <!></div> <div class="w-full space-y-2.5"><input class="sidebar-seek w-full svelte-12h25u0" type="range" min="0" step="0.5" aria-label="Seek"/> <div class="flex items-center justify-between text-[12px] text-[var(--text-muted)] numeric"><span> </span> <span class="px-3 py-0.5 rounded-full text-[11px] font-bold bg-[var(--accent-soft)] text-[var(--accent-bright)]"> </span> <span> </span></div></div> <div class="w-full flex items-center justify-center gap-4 pt-1 pb-1"><button title="Shuffle"><!> <!></button> <button class="icon-btn !w-9 !h-9" title="Previous"><!></button> <button class="play-circle !w-[56px] !h-[56px] shrink-0" title="Play / Pause"><!></button> <button class="icon-btn !w-9 !h-9" title="Next"><!></button> <button title="Repeat"><!> <!></button></div></div></div>');function fd(t,e){Je(e,!0);let r=O(!1),a=O(0),n=O(null),i=O(1),l=O(!1),c=Z(()=>s(r)?s(a):o.positionSecs),d=Z(()=>o.durationSecs>0?Math.max(0,Math.min(100,s(c)/o.durationSecs*100)):0),f=Z(()=>Math.round(o.volume*100)),h=Z(()=>o.currentTrack?o.isFavorite(o.currentTrack.id):!1);function p(Q){x(n,s(n)===Q?null:Q,!0)}function g(){o.volume>.01?(x(i,o.volume,!0),o.setVolume(0)):o.setVolume(s(i)>.05?s(i):.8)}var b=vd(),w=v(b),L=v(w);ps(L,()=>o.currentTrack?.id??"none",Q=>{var ye=rd(),qe=v(ye);{let ut=Z(()=>o.currentTrack?.cover_large_url),dt=Z(()=>o.currentTrack?.cover_url),vt=Z(()=>o.currentTrack?.title??"");Ze(qe,{class:"cover-large !w-[264px] !h-[264px] !rounded-2xl border border-white/10 shadow-[0_22px_50px_rgba(0,0,0,0.48)] transition-transform duration-300 hover:scale-[1.015]",get src(){return s(ut)},get fallbackSrc(){return s(dt)},get alt(){return s(vt)},eager:!0})}gt(1,ye,()=>pa,()=>({start:.94,duration:260,easing:Mt})),_(Q,ye)});var k=u(w,2),M=v(k),A=v(M),z=N(A,!0),q=u(A,2),C=N(q,!0),$=u(q,2);{var m=Q=>{var ye=ad(),qe=N(ye);U(()=>I(qe,`Album: ${o.currentTrack.album??""}`)),_(Q,ye)};F($,Q=>{o.currentTrack?.album&&Q(m)})}var P=u(M,2),D=v(P),T=v(D);{var j=Q=>{Us(Q,{size:18})},R=Q=>{Qs(Q,{size:18})},G=Q=>{ha(Q,{size:18})};F(T,Q=>{o.volume===0?Q(j):o.volume<.5?Q(R,1):Q(G,-1)})}var H=u(D,2),K=v(H);{let Q=Z(()=>s(h)?"currentColor":"none");Cr(K,{size:18,get fill(){return s(Q)}})}var ce=u(H,2),ne=v(ce);{var Y=Q=>{var ye=sd();_(Q,ye)},ue=Q=>{Cl(Q,{size:18})};F(ne,Q=>{s(l)?Q(Y):Q(ue,-1)})}var le=u(ce,2),ge=v(le);jl(ge,{size:18});var V=u(ge,2);{var ie=Q=>{var ye=nd();_(Q,ye)};F(V,Q=>{e.sleepRemainingSecs!=null&&Q(ie)})}var W=u(le,2),ee=v(W);_s(ee,{size:18});var E=u(W,2);{var J=Q=>{var ye=id();y("click",ye,()=>x(n,null)),_(Q,ye)};F(E,Q=>{s(n)!==null&&Q(J)})}var te=u(E,2);{var Le=Q=>{var ye=od(),qe=v(ye),ut=u(v(qe),2),dt=N(ut),vt=u(qe,2),et=v(vt),wt=v(et);{var Bt=mt=>{Us(mt,{size:16})},tt=mt=>{Qs(mt,{size:16})},Gt=mt=>{ha(mt,{size:16})};F(wt,mt=>{o.volume===0?mt(Bt):o.volume<.5?mt(tt,1):mt(Gt,-1)})}var cr=u(et,2);U(()=>{I(dt,`${s(f)??""}%`),X(et,1,`icon-btn !w-8 !h-8 shrink-0 bg-[var(--surface)] hover:bg-[var(--surface-hover)] ${o.volume===0?"text-[var(--coral)]":"text-[var(--text)]"}`),it(et,"title",o.volume===0?"Unmute":"Mute"),es(cr,o.volume),lt(cr,`background: linear-gradient(to right, var(--accent-bright) ${s(f)??""}%, rgba(255,255,255,0.16) ${s(f)??""}%);`)}),y("click",et,g),y("input",cr,mt=>o.setVolume(Number(mt.currentTarget.value))),gt(3,ye,()=>tr,()=>({y:8,duration:170,easing:Mt})),_(Q,ye)},he=Q=>{var ye=ud(),qe=v(ye),ut=u(v(qe),2);{var dt=et=>{var wt=ld(),Bt=N(wt,!0);U(tt=>I(Bt,tt),[()=>St(e.sleepRemainingSecs)]),_(et,wt)};F(ut,et=>{e.sleepRemainingSecs!=null&&et(dt)})}var vt=u(qe,2);Ve(vt,20,()=>[null,15,30,45,60],_r,(et,wt)=>{var Bt=cd(),tt=N(Bt,!0);U(()=>I(tt,wt==null?"Off":`${wt}m`)),y("click",Bt,()=>{e.onSetSleepTimer(wt),x(n,null)}),_(et,Bt)}),gt(3,ye,()=>tr,()=>({y:8,duration:170,easing:Mt})),_(Q,ye)};F(te,Q=>{s(n)==="volume"?Q(Le):s(n)==="timer"&&Q(he,1)})}var Oe=u(P,2),re=v(Oe),ve=u(re,2),Ie=v(ve),Ce=N(Ie,!0),Ne=u(Ie,2),je=N(Ne,!0),Ee=u(Ne,2),be=N(Ee,!0),pe=u(Oe,2),ze=v(pe),Qe=v(ze);gi(Qe,{size:17});var me=u(Qe,2);{var ae=Q=>{var ye=tn();_(Q,ye)};F(me,Q=>{o.shuffle&&Q(ae)})}var oe=u(ze,2),_e=v(oe);bs(_e,{size:20,fill:"currentColor"});var Se=u(oe,2),We=v(Se);{var Fe=Q=>{var ye=dd();_(Q,ye)},Ue=Q=>{gs(Q,{size:22,fill:"currentColor"})},B=Q=>{Rr(Q,{size:22,fill:"currentColor",class:"translate-x-[1.5px]"})};F(We,Q=>{o.isLoadingAudio?Q(Fe):o.isPlaying?Q(Ue,1):Q(B,-1)})}var de=u(Se,2),Re=v(de);ms(Re,{size:20,fill:"currentColor"});var Be=u(de,2),_t=v(Be);_i(_t,{size:17});var ct=u(_t,2);{var st=Q=>{var ye=tn();_(Q,ye)};F(ct,Q=>{o.repeat&&Q(st)})}U((Q,ye,qe,ut)=>{I(z,o.currentTrack?.title??"Nothing playing"),I(C,o.currentTrack?.artist??"Select a song"),X(D,1,`icon-btn !w-9 !h-9 ${s(n)==="volume"||o.volume===0?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),it(D,"title",`Volume (${s(f)??""}%)`),X(H,1,`icon-btn !w-9 !h-9 transition-transform active:scale-90 ${s(h)?"text-[var(--coral)]":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),H.disabled=!o.currentTrack,it(H,"title",s(h)?"Remove from favorites":"Add to favorites"),X(ce,1,`icon-btn !w-9 !h-9 ${s(l)?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),ce.disabled=!o.currentTrack||s(l),X(le,1,`icon-btn !w-9 !h-9 relative ${s(n)==="timer"||e.sleepRemainingSecs!=null?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),it(le,"title",Q),X(W,1,`icon-btn !w-9 !h-9 ${o.partyMode?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),it(re,"max",ye),es(re,s(c)),re.disabled=!o.currentTrack,lt(re,`background: linear-gradient(to right, var(--accent-bright) ${s(d)??""}%, rgba(255,255,255,0.16) ${s(d)??""}%);`),I(Ce,qe),I(je,o.qualityLabel),I(be,ut),X(ze,1,`icon-btn relative ${o.shuffle?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),oe.disabled=!o.currentTrack,Se.disabled=!o.currentTrack,de.disabled=!o.currentTrack,X(Be,1,`icon-btn relative ${o.repeat?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`)},[()=>e.sleepRemainingSecs!=null?`Sleep timer: ${St(e.sleepRemainingSecs)}`:"Sleep timer",()=>Math.max(1,o.durationSecs),()=>St(s(c)),()=>St(o.durationSecs)]),y("click",D,()=>p("volume")),y("click",H,()=>o.currentTrack&&o.toggleFavorite(o.currentTrack)),y("click",ce,async()=>{x(n,null),x(l,!0);try{await o.startRadioFromCurrent()}finally{x(l,!1)}}),y("click",le,()=>p("timer")),y("click",W,()=>{x(n,null),o.partyMode=!0}),y("pointerdown",re,()=>{x(r,!0),x(a,o.positionSecs,!0)}),y("input",re,Q=>{x(a,Number(Q.currentTarget.value),!0)}),y("change",re,Q=>{const ye=Number(Q.currentTarget.value);x(r,!1),o.seek(ye)}),y("click",ze,()=>o.toggleShuffle()),y("click",oe,()=>void o.prev()),y("click",Se,()=>void o.togglePlay()),y("click",de,()=>void o.next()),y("click",Be,()=>o.toggleRepeat()),gt(1,b,()=>Pa,()=>({duration:180})),_(t,b),Xe()}at(["click","input","pointerdown","change"]);var hd=S('<button class="px-2.5 py-1 rounded-full text-[11.5px] font-semibold bg-[var(--surface)] hover:bg-red-500/20 hover:text-red-400 transition-colors svelte-1567ky0">Clear</button>'),pd=S('<div class="flex flex-col items-center justify-center text-center gap-2 py-10 text-[var(--text-muted)] svelte-1567ky0"><!> <p class="text-[13px] svelte-1567ky0">The queue is empty.</p></div>'),_d=S('<span class="eq-bars shrink-0 svelte-1567ky0" aria-label="Playing"><span class="svelte-1567ky0"></span><span class="svelte-1567ky0"></span><span class="svelte-1567ky0"></span></span>'),gd=S('<span class="text-[11px] text-[var(--text-muted)] numeric shrink-0 svelte-1567ky0"> </span>'),bd=S('<div draggable="true" role="button" tabindex="0"><!> <!> <div class="track-meta svelte-1567ky0"><span> </span> <span class="track-sub !text-[11px] svelte-1567ky0"> </span></div> <!> <button class="icon-btn !w-6 !h-6 opacity-0 group-hover:opacity-100 transition-opacity shrink-0 svelte-1567ky0" title="Remove from queue"><!></button></div>'),md=S('<div class="track-list svelte-1567ky0"></div>'),xd=S('<div class="flex items-center gap-2.5 px-2.5 py-2 border-b border-[var(--border)] last:border-b-0 hover:bg-[var(--surface-hover)] transition-colors svelte-1567ky0"><!> <div class="track-meta svelte-1567ky0"><span class="track-title !text-[12.5px] svelte-1567ky0"> </span> <span class="track-sub !text-[11px] svelte-1567ky0"> </span></div> <button class="icon-btn !w-7 !h-7 shrink-0 svelte-1567ky0" title="Add to queue"><!></button></div>'),yd=S('<h4 class="text-[11.5px] font-bold uppercase tracking-wider text-[var(--text-muted)] mt-1 svelte-1567ky0">Up next suggestions</h4> <div class="track-list svelte-1567ky0"></div>',1),wd=S('<div class="flex flex-col gap-3 pt-1 svelte-1567ky0"><div class="flex items-center justify-between gap-2 svelte-1567ky0"><h3 class="text-[14px] font-bold svelte-1567ky0">Queue <span class="text-[var(--text-muted)] font-medium svelte-1567ky0"> </span></h3> <!></div> <!> <div class="flex items-center justify-between gap-3 p-3 rounded-xl bg-[var(--surface)] border border-[var(--border)] svelte-1567ky0"><div class="min-w-0 svelte-1567ky0"><div class="text-[13px] font-bold svelte-1567ky0">Autoplay</div> <p class="text-[11.5px] text-[var(--text-muted)] leading-snug svelte-1567ky0">Queue similar songs when queue ends</p></div> <button type="button" role="switch" aria-label="Toggle autoplay"><span></span></button></div> <!></div>');function kd(t,e){Je(e,!0);let r=O(null),a=O(null);var n=wd(),i=v(n),l=v(i),c=u(v(l)),d=N(c),f=u(l,2);{var h=z=>{var q=hd();y("click",q,()=>o.clearQueue()),_(z,q)};F(f,z=>{o.queue.length>0&&z(h)})}var p=u(i,2);{var g=z=>{var q=pd(),C=v(q);fi(C,{size:26,class:"opacity-40"}),_(z,q)},b=z=>{var q=md();Ve(q,23,()=>o.queue,(C,$)=>C.id+":"+$,(C,$,m)=>{const P=Z(()=>o.currentIndex===s(m));var D=bd(),T=v(D);Al(T,{size:13,class:"text-[var(--text-muted)]/45 cursor-grab shrink-0"});var j=u(T,2);Ze(j,{class:"track-cover !w-9 !h-9 !rounded-md",get src(){return s($).cover_url},alt:""});var R=u(j,2),G=v(R),H=N(G,!0),K=u(G,2),ce=N(K,!0),ne=u(R,2);{var Y=V=>{var ie=_d();_(V,ie)},ue=V=>{var ie=gd(),W=N(ie,!0);U(ee=>I(W,ee),[()=>St(s($).duration_secs)]),_(V,ie)};F(ne,V=>{s(P)&&o.isPlaying?V(Y):V(ue,-1)})}var le=u(ne,2),ge=v(le);xs(ge,{size:13}),U(()=>{X(D,1,`group flex items-center gap-2 px-2.5 py-2 border-b border-[var(--border)] last:border-b-0 cursor-pointer transition-colors ${s(P)?"bg-[var(--accent-soft)]":"hover:bg-[var(--surface-hover)]"} ${s(a)===s(m)?"border-t-2 border-t-[var(--accent-bright)]":""}`,"svelte-1567ky0"),X(G,1,`track-title !text-[12.5px] ${s(P)?"!text-[var(--accent-bright)] !font-bold":""}`,"svelte-1567ky0"),I(H,s($).title),I(ce,s($).artist)}),xt("dragstart",D,()=>x(r,s(m),!0)),xt("dragover",D,V=>{V.preventDefault(),x(a,s(m),!0)}),xt("dragleave",D,()=>{s(a)===s(m)&&x(a,null)}),xt("drop",D,()=>{s(r)!=null&&o.moveQueueItem(s(r),s(m)),x(r,null),x(a,null)}),xt("dragend",D,()=>{x(r,null),x(a,null)}),y("click",D,()=>void o.playIndex(s(m))),y("keydown",D,V=>V.key==="Enter"&&void o.playIndex(s(m))),y("click",le,V=>{V.stopPropagation(),o.removeQueueIndex(s(m))}),_(C,D)}),_(z,q)};F(p,z=>{o.queue.length===0?z(g):z(b,-1)})}var w=u(p,2),L=u(v(w),2),k=N(L),M=u(w,2);{var A=z=>{var q=yd(),C=u(se(q),2);Ve(C,21,()=>o.suggestedTracks,$=>$.id,($,m)=>{var P=xd(),D=v(P);Ze(D,{class:"track-cover !w-9 !h-9 !rounded-md",get src(){return s(m).cover_url},alt:""});var T=u(D,2),j=v(T),R=N(j,!0),G=u(j,2),H=N(G,!0),K=u(T,2),ce=v(K);pi(ce,{size:15}),U(()=>{I(R,s(m).title),I(H,s(m).artist)}),y("click",K,()=>o.enqueueTrack(s(m))),_($,P)}),_(z,q)};F(M,z=>{o.suggestedTracks.length>0&&z(A)})}U(()=>{I(d,`· ${o.queue.length??""}`),it(L,"aria-checked",o.autoplayRelated),X(L,1,`w-10 h-6 rounded-full p-0.5 transition-colors shrink-0 ${o.autoplayRelated?"bg-[var(--accent)]":"bg-white/15"}`,"svelte-1567ky0"),X(k,1,`block w-5 h-5 rounded-full bg-white shadow transition-transform ${o.autoplayRelated?"translate-x-4":"translate-x-0"}`,"svelte-1567ky0")}),y("click",L,()=>o.setAutoplay(!o.autoplayRelated)),gt(1,n,()=>Pa,()=>({duration:180})),_(t,n),Xe()}at(["click","keydown"]);var $d=S('<aside class="w-[388px] shrink-0 h-full flex flex-col border-r border-[var(--border)] bg-[var(--bg-sidebar)] overflow-hidden relative select-none" style="background-image: linear-gradient(180deg, rgba(var(--ambient-rgb), 0.22) 0%, rgba(var(--ambient-rgb), 0.08) 52%, transparent 100%);"><div class="shrink-0 flex items-center justify-between gap-2 px-6 pt-4 pb-2"><div class="min-w-0 flex flex-col leading-tight"><span class="text-[11.5px] font-medium text-[var(--text-muted)]">Playing from</span> <span class="text-[13px] font-semibold truncate"> </span></div> <button class="icon-btn shrink-0 text-[var(--text-muted)] hover:text-[var(--text)]" title="Collapse player"><!></button></div> <div><!></div> <nav class="shrink-0 grid grid-cols-3 gap-1.5 px-3 py-2.5 border-t border-[var(--border)] bg-[var(--bg-sidebar)]"><button><!> Player</button> <button><!> Lyrics</button> <button><!> Queue</button></nav></aside>');function Sd(t,e){Je(e,!0);let r=O(null),a;function n(P){if(window.clearInterval(a),a=void 0,!P){x(r,null);return}x(r,P*60),o.showToast(`Sleep timer set for ${P} min`),a=window.setInterval(()=>{s(r)!=null&&(x(r,s(r)-1),s(r)<=0&&(window.clearInterval(a),a=void 0,x(r,null),o.isPlaying&&o.togglePlay(),o.showToast("Sleep timer ended — playback paused")))},1e3)}var i=$d(),l=v(i),c=v(l),d=u(v(c),2),f=N(d,!0),h=u(c,2),p=v(h);ts(p,{size:19});var g=u(l,2),b=v(g);{var w=P=>{fd(P,{get sleepRemainingSecs(){return s(r)},onSetSleepTimer:n})},L=P=>{td(P,{})},k=P=>{kd(P,{})};F(b,P=>{o.sidebarTab==="player"?P(w):o.sidebarTab==="lyrics"?P(L,1):P(k,-1)})}var M=u(g,2),A=v(M),z=v(A);Ta(z,{size:15});var q=u(A,2),C=v(q);Nl(C,{size:15});var $=u(q,2),m=v($);fi(m,{size:15}),U(()=>{I(f,o.sourceName),X(g,1,`flex-1 min-h-0 px-7 pb-4 flex flex-col ${o.sidebarTab==="lyrics"?"overflow-hidden":"overflow-y-auto scroll-y"}`),X(A,1,`flex items-center justify-center gap-1.5 py-2 rounded-xl text-[13px] font-semibold border transition-colors ${o.sidebarTab==="player"?"bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]":"border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]"}`),X(q,1,`flex items-center justify-center gap-1.5 py-2 rounded-xl text-[13px] font-semibold border transition-colors ${o.sidebarTab==="lyrics"?"bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]":"border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]"}`),X($,1,`flex items-center justify-center gap-1.5 py-2 rounded-xl text-[13px] font-semibold border transition-colors ${o.sidebarTab==="queue"?"bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]":"border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]"}`)}),y("click",h,()=>{o.sidebarOpen=!1,o.persistSession()}),y("click",A,()=>o.sidebarTab="player"),y("click",q,()=>o.sidebarTab="lyrics"),y("click",$,()=>o.sidebarTab="queue"),_(t,i),Xe()}at(["click"]);var Td=S('<span class="live-pill svelte-ja6svg">● Live</span>'),rn=S('<div class="qr-box svelte-ja6svg"></div>'),Pd=S('<div class="spin-card qr-section svelte-ja6svg"><span class="qr-kicker svelte-ja6svg">Scan to join</span> <!> <div class="copy-row svelte-ja6svg"><code class="svelte-ja6svg"> </code> <button class="pill-btn">Copy</button></div></div>'),Ad=S('<div class="spin-card svelte-ja6svg"><label class="switch-row svelte-ja6svg"><span class="svelte-ja6svg"><strong>Enable local Jukebox</strong> <small class="muted svelte-ja6svg">Local web server on your Wi-Fi</small></span> <input type="checkbox"/></label> <label class="switch-row svelte-ja6svg"><span class="svelte-ja6svg"><strong>Allow controls</strong> <small class="muted svelte-ja6svg">Pause, skip, and reorder queue</small></span> <input type="checkbox"/></label> <div class="status-line svelte-ja6svg"><span class="muted">Status</span> <span> </span></div></div> <!>',1),Ed=S('<div class="spin-card qr-section svelte-ja6svg"><span class="qr-kicker svelte-ja6svg">Active invitation</span> <!> <div class="copy-row svelte-ja6svg"><code class="svelte-ja6svg"> </code> <button class="pill-btn">Copy</button></div></div>'),Md=S('<div class="spin-card svelte-ja6svg"><label class="switch-row svelte-ja6svg"><span class="svelte-ja6svg"><strong>Broadcast as host</strong> <small class="muted svelte-ja6svg">Create a synced listening invitation</small></span> <input type="checkbox"/></label></div> <!> <div class="spin-card join-card svelte-ja6svg"><strong>Join a room</strong> <p class="muted svelte-ja6svg">Paste a SPIN invitation to listen in sync.</p> <input class="text-input svelte-ja6svg" type="text" placeholder="GYRO-… or host URL"/> <button> </button></div>',1),Id=S('<aside class="spin-sidebar w-[340px] shrink-0 h-full flex flex-col border-l border-[var(--border)] bg-[var(--bg-sidebar)] overflow-hidden max-xl:fixed max-xl:right-0 max-xl:top-14 max-xl:bottom-0 max-xl:z-40 max-xl:shadow-2xl svelte-ja6svg"><div class="spin-header svelte-ja6svg"><div class="spin-title-row svelte-ja6svg"><h3 class="svelte-ja6svg">SPIN Mode</h3> <!></div> <button class="icon-btn" title="Close SPIN panel">✕</button></div> <div class="spin-body scroll-y svelte-ja6svg"><div class="mode-switch svelte-ja6svg"><button>Wi-Fi Jukebox</button> <button>App to App</button></div> <!></div></aside>');function Nd(t,e){Je(e,!0);let r=O("local"),a=O(""),n=O(!1),i=Z(()=>o.spinStatus.local_active||o.spinStatus.remote_role!=="idle");async function l(T){if(He()){x(n,!0);try{o.spinStatus=T?await Te.spinStartLocal():await Te.spinStopLocal()}catch(j){o.showToast(String(j))}finally{x(n,!1)}}}async function c(T){if(He())try{o.spinStatus=await Te.spinSetAllowControls(T)}catch(j){o.showToast(String(j))}}async function d(T){if(He()){x(n,!0);try{o.spinStatus=T?await Te.spinStartRemoteHost():await Te.spinStopRemote()}catch(j){o.showToast(String(j))}finally{x(n,!1)}}}async function f(){if(He()){x(n,!0);try{o.spinStatus.remote_role==="guest"?o.spinStatus=await Te.spinStopRemote():(o.spinStatus=await Te.spinJoinRemote(s(a).trim()),o.showToast("Connecting to SPIN room…"))}catch(T){o.showToast(String(T))}finally{x(n,!1)}}}async function h(T,j){try{await navigator.clipboard.writeText(T),o.showToast(j)}catch{o.showToast("Could not copy to clipboard")}}var p=Id(),g=v(p),b=v(g),w=u(v(b),2);{var L=T=>{var j=Td();_(T,j)};F(w,T=>{s(i)&&T(L)})}var k=u(b,2),M=u(g,2),A=v(M),z=v(A);let q;var C=u(z,2);let $;var m=u(A,2);{var P=T=>{var j=Ad(),R=se(j),G=v(R),H=u(v(G),2),K=u(G,2),ce=u(v(K),2),ne=u(K,2),Y=u(v(ne),2),ue=N(Y,!0),le=u(R,2);{var ge=V=>{var ie=Pd(),W=u(v(ie),2);{var ee=he=>{var Oe=rn();zs(Oe,()=>o.spinStatus.local_qr_svg,!0),_(he,Oe)};F(W,he=>{o.spinStatus.local_qr_svg&&he(ee)})}var E=u(W,2),J=v(E),te=N(J,!0),Le=u(J,2);U(()=>I(te,o.spinStatus.local_url)),y("click",Le,()=>void h(o.spinStatus.local_url,"SPIN link copied")),_(V,ie)};F(le,V=>{o.spinStatus.local_active&&V(ge)})}U(()=>{H.disabled=s(n),Wr(H,o.spinStatus.local_active),Wr(ce,o.spinStatus.allow_controls),I(ue,o.spinStatus.local_active?`${o.spinStatus.guest_count} guests • ${o.spinStatus.added_count} added`:"Inactive")}),y("change",H,V=>void l(V.currentTarget.checked)),y("change",ce,V=>void c(V.currentTarget.checked)),_(T,j)},D=T=>{var j=Md(),R=se(j),G=v(R),H=u(v(G),2),K=u(R,2);{var ce=V=>{var ie=Ed(),W=u(v(ie),2);{var ee=he=>{var Oe=rn();zs(Oe,()=>o.spinStatus.remote_qr_svg,!0),_(he,Oe)};F(W,he=>{o.spinStatus.remote_qr_svg&&he(ee)})}var E=u(W,2),J=v(E),te=N(J,!0),Le=u(J,2);U(()=>I(te,o.spinStatus.remote_ticket)),y("click",Le,()=>void h(o.spinStatus.remote_ticket??"","Invitation copied")),_(V,ie)};F(K,V=>{o.spinStatus.remote_role==="host"&&o.spinStatus.remote_ticket&&V(ce)})}var ne=u(K,2),Y=u(v(ne),4),ue=u(Y,2);let le;var ge=N(ue,!0);U(()=>{H.disabled=s(n),Wr(H,o.spinStatus.remote_role==="host"),le=X(ue,1,"pill-btn svelte-ja6svg",null,le,{primary:o.spinStatus.remote_role!=="guest"}),ue.disabled=s(n),I(ge,o.spinStatus.remote_role==="guest"?"Disconnect":"Join")}),y("change",H,V=>void d(V.currentTarget.checked)),fa(Y,()=>s(a),V=>x(a,V)),y("click",ue,()=>void f()),_(T,j)};F(m,T=>{s(r)==="local"?T(P):T(D,-1)})}U(()=>{q=X(z,1,"svelte-ja6svg",null,q,{active:s(r)==="local"}),$=X(C,1,"svelte-ja6svg",null,$,{active:s(r)==="remote"})}),y("click",k,()=>o.spinOpen=!1),y("click",z,()=>x(r,"local")),y("click",C,()=>x(r,"remote")),_(t,p),Xe()}at(["click","change"]);var an=S('<div class="h-full shrink-0"><!></div>'),sn=S("<div><!></div>"),zd=S('<div class="fixed bottom-5 left-1/2 -translate-x-1/2 z-40 w-[min(560px,calc(100vw-32px))] flex items-center gap-2.5 px-3.5 py-2 rounded-full bg-[var(--bg-elevated)]/95 backdrop-blur-xl border border-[var(--border-strong)] shadow-2xl"><button class="flex-1 min-w-0 flex items-center gap-2.5 text-left"><!> <div class="track-meta"><span class="track-title"> </span> <span class="track-sub"> </span></div></button> <button title="Favorite"><!></button> <button class="icon-btn" title="Previous"><!></button> <button class="play-circle !w-10 !h-10" title="Play / Pause"><!></button> <button class="icon-btn" title="Next"><!></button></div>'),Ld=S('<div class="px-4 py-2 rounded-full text-[13px] font-medium bg-[var(--bg-elevated)]/95 backdrop-blur-md border border-[var(--border-strong)] shadow-xl"> </div>'),Cd=S('<div class="fixed bottom-5 left-1/2 -translate-x-1/2 z-50 flex flex-col items-center gap-2 pointer-events-none" aria-live="polite"></div>'),Rd=S('<div class="h-screen w-screen overflow-hidden flex flex-col bg-[var(--bg)] text-[var(--text)]"><!> <div class="flex-1 min-h-0 flex overflow-hidden relative"><!> <main class="flex-1 min-w-0 h-full overflow-y-auto ambient-surface scroll-y"><nav class="flex items-center justify-center gap-2 pt-3.5 pb-1 px-4"><button><!> Home</button> <button><!> Explore</button> <button><!> Collection</button></nav> <!></main> <!></div> <!> <!> <!> <!></div>');function qd(t,e){Je(e,!0),Sa(()=>{o.bootstrap()});let r=Z(()=>o.activeView.kind==="artist"?`artist:${o.activeView.artistId}`:o.activeView.kind==="album"?`album:${o.activeView.id}`:o.activeView.kind);At(()=>{const T=o.ambientRgb;document.documentElement.style.setProperty("--ambient-rgb",T?`${T[0]}, ${T[1]}, ${T[2]}`:"33, 144, 164")});var a=Rd(),n=v(a);vu(n,{});var i=u(n,2),l=v(i);{var c=T=>{var j=an(),R=v(j);Sd(R,{}),gt(3,j,()=>tr,()=>({x:-24,duration:220,easing:Mt})),_(T,j)};F(l,T=>{o.sidebarOpen&&T(c)})}var d=u(l,2),f=v(d),h=v(f),p=v(h);Ml(p,{size:15});var g=u(h,2),b=v(g);vi(b,{size:15});var w=u(g,2),L=v(w);Il(L,{size:15});var k=u(f,2);ps(k,()=>s(r),T=>{var j=sn(),R=v(j);{var G=Y=>{Nu(Y,{})},H=Y=>{au(Y,{})},K=Y=>{Dc(Y,{})},ce=Y=>{Sc(Y,{get artistId(){return o.activeView.artistId},get artistName(){return o.activeView.artistName}})},ne=Y=>{oc(Y,{get id(){return o.activeView.id},get title(){return o.activeView.title},get coverUrl(){return o.activeView.coverUrl},get isPlaylist(){return o.activeView.isPlaylist}})};F(R,Y=>{o.activeView.kind==="home"?Y(G):o.activeView.kind==="explore"?Y(H,1):o.activeView.kind==="collection"?Y(K,2):o.activeView.kind==="artist"?Y(ce,3):o.activeView.kind==="album"&&Y(ne,4)})}gt(1,j,()=>tr,()=>({y:10,duration:220,easing:Mt})),_(T,j)});var M=u(d,2);{var A=T=>{var j=an(),R=v(j);Nd(R,{}),gt(3,j,()=>tr,()=>({x:24,duration:220,easing:Mt})),_(T,j)};F(M,T=>{o.spinOpen&&T(A)})}var z=u(i,2);{var q=T=>{var j=zd(),R=v(j),G=v(R);Ze(G,{class:"w-10 h-10 rounded-lg object-cover shrink-0",get src(){return o.currentTrack.cover_url},alt:""});var H=u(G,2),K=v(H),ce=N(K,!0),ne=u(K,2),Y=N(ne,!0),ue=u(R,2);let le;var ge=v(ue);{let he=Z(()=>o.isFavorite(o.currentTrack.id)?"currentColor":"none");Cr(ge,{size:17,get fill(){return s(he)}})}var V=u(ue,2),ie=v(V);bs(ie,{size:17,fill:"currentColor"});var W=u(V,2),ee=v(W);{var E=he=>{gs(he,{size:17,fill:"currentColor"})},J=he=>{Rr(he,{size:17,fill:"currentColor",class:"translate-x-[1px]"})};F(ee,he=>{o.isPlaying?he(E):he(J,-1)})}var te=u(W,2),Le=v(te);ms(Le,{size:17,fill:"currentColor"}),U(he=>{I(ce,o.currentTrack.title),I(Y,o.currentTrack.artist),le=X(ue,1,"icon-btn",null,le,{"fav-active":he})},[()=>o.isFavorite(o.currentTrack.id)]),y("click",R,()=>o.sidebarOpen=!0),y("click",ue,()=>o.currentTrack&&o.toggleFavorite(o.currentTrack)),y("click",V,()=>void o.prev()),y("click",W,()=>void o.togglePlay()),y("click",te,()=>void o.next()),gt(3,j,()=>tr,()=>({y:24,duration:240,easing:Mt})),_(T,j)};F(z,T=>{!o.sidebarOpen&&o.currentTrack&&T(q)})}var C=u(z,2);{var $=T=>{var j=sn(),R=v(j);Vu(R,{}),gt(3,j,()=>Pa,()=>({duration:220})),_(T,j)};F(C,T=>{o.partyMode&&T($)})}var m=u(C,2);Uu(m,{});var P=u(m,2);{var D=T=>{var j=Cd();Ve(j,21,()=>o.toasts,R=>R.id,(R,G)=>{var H=Ld(),K=N(H,!0);U(()=>I(K,s(G).message)),gt(3,H,()=>tr,()=>({y:14,duration:200,easing:Mt})),_(R,H)}),_(T,j)};F(P,T=>{o.toasts.length>0&&T(D)})}U(()=>{X(h,1,`inline-flex items-center gap-1.5 px-4 py-1.5 rounded-full text-[13px] font-semibold transition-all duration-200 active:scale-95 ${o.activeTab==="home"?"bg-[var(--surface-hover)] text-[var(--text)] shadow-sm":"text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]"}`),X(g,1,`inline-flex items-center gap-1.5 px-4 py-1.5 rounded-full text-[13px] font-semibold transition-all duration-200 active:scale-95 ${o.activeTab==="explore"?"bg-[var(--surface-hover)] text-[var(--text)] shadow-sm":"text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]"}`),X(w,1,`inline-flex items-center gap-1.5 px-4 py-1.5 rounded-full text-[13px] font-semibold transition-all duration-200 active:scale-95 ${o.activeTab==="collection"?"bg-[var(--surface-hover)] text-[var(--text)] shadow-sm":"text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]"}`)}),y("click",h,()=>o.navigateTab("home")),y("click",g,()=>o.navigateTab("explore")),y("click",w,()=>o.navigateTab("collection")),_(t,a),Xe()}at(["click"]);Yo(qd,{target:document.getElementById("app")});
