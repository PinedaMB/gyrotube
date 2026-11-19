(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const n of document.querySelectorAll('link[rel="modulepreload"]'))s(n);new MutationObserver(n=>{for(const i of n)if(i.type==="childList")for(const o of i.addedNodes)o.tagName==="LINK"&&o.rel==="modulepreload"&&s(o)}).observe(document,{childList:!0,subtree:!0});function r(n){const i={};return n.integrity&&(i.integrity=n.integrity),n.referrerPolicy&&(i.referrerPolicy=n.referrerPolicy),n.crossOrigin==="use-credentials"?i.credentials="include":n.crossOrigin==="anonymous"?i.credentials="omit":i.credentials="same-origin",i}function s(n){if(n.ep)return;n.ep=!0;const i=r(n);fetch(n.href,i)}})();const fi=!1;var ds=Array.isArray,hi=Array.prototype.indexOf,Wr=Array.prototype.includes,fs=Array.from,Ua=Object.defineProperty,Jt=Object.getOwnPropertyDescriptor,Wa=Object.getOwnPropertyDescriptors,pi=Object.prototype,_i=Array.prototype,Ks=Object.getPrototypeOf,da=Object.isExtensible;function zr(t){return typeof t=="function"}const gi=()=>{};function bi(t){return t()}function Ls(t){for(var e=0;e<t.length;e++)t[e]()}function Ba(){var t,e,r=new Promise((s,n)=>{t=s,e=n});return{promise:r,resolve:t,reject:e}}function mi(t,e){if(Array.isArray(t))return t;if(!(Symbol.iterator in t))return Array.from(t);const r=[];for(const s of t)if(r.push(s),r.length===e)break;return r}const st=2,yr=4,Yr=8,Js=1<<24,At=16,jt=32,Zt=64,Rs=128,Xs=256,Nt=512,Ze=1024,Ve=2048,gt=4096,it=8192,et=16384,$r=32768,Br=1<<25,hr=65536,ns=1<<17,yi=1<<18,Tr=1<<19,Ga=1<<20,Ot=1<<25,is=1<<21,br=1<<22,Xt=1<<23,Dt=Symbol("$state"),Ya=Symbol("component"),Ka=Symbol("legacy props"),xi=Symbol(""),Ja=Symbol("attributes"),Cs=Symbol("class"),Os=Symbol("style"),Fs=Symbol("text"),ss=Symbol("form reset"),Kr=new class extends Error{name="StaleReactionError";message="The reaction that called `getAbortSignal()` was re-run or destroyed"},hs=!!globalThis.document?.contentType&&globalThis.document.contentType.includes("xml"),ki=1,wi=2,Xa=4,Si=8,$i=16,Ti=1,Pi=2,Za=4,Ai=8,Ei=16,Ii=1,Mi=2,en={},Je=Symbol("uninitialized"),tn="http://www.w3.org/1999/xhtml",rn="http://www.w3.org/2000/svg",Ni="http://www.w3.org/1998/Math/MathML",ji="@attach";function qi(){console.warn("https://svelte.dev/e/derived_inert")}function zi(){console.warn("https://svelte.dev/e/select_multiple_invalid_value")}function Li(){console.warn("https://svelte.dev/e/svelte_boundary_reset_noop")}let sn=!1;function an(t){return t===this.v}function Ri(t,e){return t!=t?e==e:t!==e||t!==null&&typeof t=="object"||typeof t=="function"}function nn(t){return!Ri(t,this.v)}function Ci(t){throw new Error("https://svelte.dev/e/lifecycle_outside_component")}function Oi(){throw new Error("https://svelte.dev/e/async_derived_orphan")}function Fi(t,e,r){throw new Error("https://svelte.dev/e/each_key_duplicate")}function Di(t){throw new Error("https://svelte.dev/e/effect_in_teardown")}function Vi(){throw new Error("https://svelte.dev/e/effect_in_unowned_derived")}function Hi(t){throw new Error("https://svelte.dev/e/effect_orphan")}function Qi(){throw new Error("https://svelte.dev/e/effect_update_depth_exceeded")}function Ui(t){throw new Error("https://svelte.dev/e/props_invalid_value")}function Wi(){throw new Error("https://svelte.dev/e/state_descriptors_fixed")}function Bi(){throw new Error("https://svelte.dev/e/state_prototype_fixed")}function Gi(){throw new Error("https://svelte.dev/e/state_unsafe_mutation")}function Yi(){throw new Error("https://svelte.dev/e/svelte_boundary_reset_onerror")}let Pr=!1,Ki=!1;function Ji(){Pr=!0}let He=null;function xr(t){He=t}function ft(t,e=!1,r){He={p:He,i:!1,c:null,e:null,s:t,x:null,r:$e,l:Pr&&!e?{s:null,u:null,$:[]}:null}}function ht(t){var e=He,r=e.e;if(r!==null){e.e=null;for(var s of r)Sn(s)}return e.i=!0,He=e.p,Zs(t)}function Zs(t={}){return Ua(t,Ya,{value:!0}),t}function Jr(){return!Pr||He!==null&&He.l===null}let cr=[];function ln(){var t=cr;cr=[],Ls(t)}function Ft(t){if(cr.length===0&&!Vr){var e=cr;queueMicrotask(()=>{e===cr&&ln()})}cr.push(t)}function Xi(){for(;cr.length>0;)ln()}const Zi=-7169;function Ye(t,e){t.f=t.f&Zi|e}function ea(t){(t.f&Nt)!==0||t.deps===null?Ye(t,Ze):Ye(t,gt)}function on(t,e){(t.f&Ve)!==0?e.set(t,Ve):(t.f&gt)!==0&&e.get(t)!==Ve&&e.set(t,gt),Ye(t,Ze)}function el(t,e){if(e){const r=document.body;t.autofocus=!0,Ft(()=>{document.activeElement===r&&t.focus()})}}let fa=!1;function tl(){fa||(fa=!0,document.addEventListener("reset",t=>{Promise.resolve().then(()=>{if(!t.defaultPrevented)for(const e of t.target.elements)e[ss]?.()})},{capture:!0}))}function Ar(t){var e=Ee,r=$e;yt(null),xt(null);try{return t()}finally{yt(e),xt(r)}}function rl(t,e,r,s=r){t.addEventListener(e,()=>Ar(r));const n=t[ss];n?t[ss]=()=>{n(),s(!0)}:t[ss]=()=>s(!0),tl()}function cn(t,e,r,s){const n=Jr()?kr:ta;var i=t.filter(y=>!y.settled),o=e.map(n);if(r.length===0&&i.length===0){s(o);return}var c=$e,v=sl(),d=i.length===1?i[0].promise:i.length>1?Promise.all(i.map(y=>y.promise)):null;function h(y){if((c.f&et)===0){v();try{s([...o,...y])}catch(x){Rt(x,c)}ls()}}var p=un();if(r.length===0){d.then(()=>h([])).finally(p);return}function b(){Promise.all(r.map(y=>al(y))).then(h).catch(y=>Rt(y,c)).finally(p)}d?d.then(()=>{if((c.f&et)!==0){p();return}v(),b(),ls()}):b()}function sl(){var t=$e,e=Ee,r=He,s=Se;return function(i=!0){xt(t),yt(e),xr(r),i&&(t.f&et)===0&&(s?.activate(),s?.apply())}}function ls(t=!0){xt(null),yt(null),xr(null),t&&Se?.deactivate()}function un(){var t=$e,e=t.b,r=Se,s=!!e?.is_rendered();return e?.update_pending_count(1,r),r.increment(s,t),()=>{e?.update_pending_count(-1,r),r.decrement(s,t)}}function kr(t){var e=st|Ve;return $e!==null&&($e.f|=Tr),{ctx:He,deps:null,effects:null,equals:an,f:e,fn:t,reactions:null,rv:0,v:Je,wv:0,parent:$e,ac:null}}const Or=Symbol("obsolete");function al(t,e,r){let s=$e;s===null&&Oi();var n=void 0,i=tr(Je),o=!Ee,c=new Set;return bl(()=>{var v=$e,d=Ba();n=d.promise;try{Promise.resolve(t()).then(d.resolve,y=>{y!==Kr&&d.reject(y)}).finally(ls)}catch(y){d.reject(y),ls()}var h=Se;if(o){if((v.f&$r)!==0)var p=un();if(s.b?.is_rendered())h.async_deriveds.get(v)?.reject(Or);else for(const y of c.values())y.reject(Or);c.add(d),h.async_deriveds.set(v,d)}const b=(y,x=void 0)=>{p?.(),c.delete(d),x!==Or&&(h.activate(),x?(i.f|=Xt,wr(i,x)):((i.f&Xt)!==0&&(i.f^=Xt),wr(i,y)),h.deactivate())};d.promise.then(b,y=>b(null,y||"unknown"))}),_s(()=>{for(const v of c)v.reject(Or)}),new Promise(v=>{function d(h){function p(){h===n?v(i):d(n)}h.then(p,p)}d(n)})}function me(t){const e=kr(t);return Mn(e),e}function ta(t){const e=kr(t);return e.equals=nn,e}function nl(t){var e=t.effects;if(e!==null){t.effects=null;for(var r=0;r<e.length;r+=1)tt(e[r])}}function ra(t){var e,r=$e,s=t.parent;if(!Bt&&s!==null&&t.v!==Je&&(s.f&(et|it))!==0)return qi(),t.v;xt(s);try{nl(t),e=zn(t)}finally{xt(r)}return e}function vn(t){var e=ra(t);if(!t.equals(e)&&(t.wv=jn(),(!Se?.is_fork||t.deps===null)&&(Se!==null?(Se.capture(t,e,!0),mr?.capture(t,e,!0)):t.v=e,t.deps===null))){Ye(t,Ze);return}Bt||(Et!==null?(na()||Se?.is_fork)&&Et.set(t,e):ea(t))}function il(t){if(t.effects!==null)for(const e of t.effects)(e.teardown||e.ac)&&(e.teardown?.(),e.ac!==null&&Ar(()=>{e.ac.abort(Kr),e.ac=null}),e.fn!==null&&(e.teardown=gi),Gr(e,0),oa(e))}function dn(t){if(t.effects!==null)for(const e of t.effects)e.teardown&&e.fn!==null&&Sr(e)}let Ts=null,gr=null,Se=null,mr=null,Et=null,Ds=null,Vr=!1,Ps=!1,Hr=null,as=null;var ha=0;let ll=1;class er{id=ll++;started=!1;linked=!0;prev=null;next=null;async_deriveds=new Map;current=new Map;previous=new Map;#r=new Set;#o=new Set;#e=0;#i=new Map;#s=null;#t=[];#n=[];#a=new Map;#l=new Map;#f=new Set;is_fork=!1;#c=!1;constructor(){gr===null?Ts=gr=this:(gr.next=this,this.prev=gr),gr=this}#h(){if(this.is_fork)return!0;for(const e of this.#i.keys())if(!this.#g(e))return!0;return!1}#g(e){for(var r=e;r!==null;r=r.parent)if(this.#l.has(r))return!0;return!1}skip_effect(e){this.#l.has(e)||this.#l.set(e,{d:[],m:[]}),this.#f.delete(e)}unskip_effect(e,r=s=>this.schedule(s)){var s=this.#l.get(e);if(s){this.#l.delete(e);for(var n of s.d)Ye(n,Ve),r(n);for(n of s.m)Ye(n,gt),r(n)}this.#f.add(e)}#u(){var e=[];for(const i of this.#t)if(!((i.f&et)!==0||(i.f&(Ve|gt))===0)){for(var r=i,s=!1;r.parent!==null;){r=r.parent;var n=r.f;if((n&(Zt|jt))!==0){if((n&Ze)===0){s=!0;break}r.f^=Ze}}s||e.push(r)}return this.#t=[],e}#p(){this.started=!0;for(const[c,v]of this.#a)Ye(c,v),this.schedule(c);this.apply();for(var e=Hr=[],r=[],s=as=[];this.#t.length>0;){ha++>1e3&&(this.#d(),cl());for(const c of this.#u())try{this.#m(c,e,r)}catch(v){throw pn(c),this.#h()||this.discard(),v}if(this.#t.length>0){for(const c of r)this.#t.push(c);for(const c of e)this.#t.push(c);r.length=0,e.length=0}}if(Se=null,s.length>0){var n=er.ensure();for(const c of s)n.schedule(c)}if(Hr=null,as=null,this.#h()){this.#_(r),this.#_(e);for(const[c,v]of this.#l)hn(c,v);s.length>0&&Se.#p();return}const i=this.#k();if(i){this.#_(r),this.#_(e),i.#v(this);return}this.#a.clear();for(const c of this.#r)c(this);this.#r.clear(),mr=this,pa(r),pa(e),mr=null,this.#s?.resolve();var o=Se;this.#e===0&&(this.#t.length===0||o!==null)&&this.#d(),o!==null&&(Vt.clear(),o.#p())}#m(e,r,s){e.f^=Ze;for(var n=e.first;n!==null;){var i=n.f,o=(i&(jt|Zt))!==0,c=o&&(i&Ze)!==0,v=c||(i&it)!==0||this.#l.has(n);if(!v&&n.fn!==null){o?n.f^=Ze:(i&yr)!==0?r.push(n):Zr(n)&&((i&At)!==0&&this.add_dirty_reaction(n,gt),Sr(n));var d=n.first;if(d!==null){n=d;continue}}for(;n!==null;){var h=n.next;if(h!==null){n=h;break}n=n.parent}}}#k(){for(var e=this.prev;e!==null;e=e.prev)if(!e.is_fork&&this.#b(e))return e;return null}#b(e){for(const[r,[,s]]of this.current)if(!s&&e.current.has(r))return!0;return!1}#v(e){for(const[s,n]of e.current)!this.previous.has(s)&&e.previous.has(s)&&this.previous.set(s,e.previous.get(s)),this.current.set(s,n);for(const[s,n]of e.async_deriveds){const i=this.async_deriveds.get(s);i&&n.promise.then(i.resolve).catch(i.reject)}e.async_deriveds.clear(),this.transfer_reactions(e.#a);const r=s=>{var n=s.reactions;if(n!==null&&!((s.f&st)!==0&&(s.f&(Ve|gt))===0))for(const c of n){var i=c.f;if((i&st)!==0)r(c);else{var o=c;i&(br|At)&&!this.async_deriveds.has(o)&&(this.#a.delete(o),Ye(o,Ve),this.schedule(o))}}};for(const s of this.current.keys())r(s);this.oncommit(()=>e.discard()),e.#d(),Se=this,this.#p()}#_(e){for(var r=0;r<e.length;r+=1)on(e[r],this.#a)}capture(e,r,s=!1){e.v!==Je&&!this.previous.has(e)&&this.previous.set(e,e.v),(e.f&Xt)===0&&(this.current.set(e,[r,s]),Et?.set(e,r)),this.is_fork||(e.v=r)}activate(){Se=this}deactivate(){Se=null,Et=null}flush(){try{Ps=!0,Se=this,this.#p()}finally{ha=0,Ds=null,Hr=null,as=null,Ps=!1,Se=null,Et=null,Vt.clear()}}discard(){for(const e of this.#o)e(this);this.#o.clear();for(const e of this.async_deriveds.values())e.reject(Or);this.#d(),this.#s?.resolve()}register_created_effect(e){this.#n.push(e)}#y(){for(let p=Ts;p!==null;p=p.next){var e=p.id<this.id,r=[];for(const[b,[y,x]]of this.current){if(p.current.has(b)){var s=p.current.get(b)[0];if(e&&y!==s)p.current.set(b,[y,x]);else continue}r.push(b)}if(e)for(const[b,y]of this.async_deriveds){const x=p.async_deriveds.get(b);x&&y.promise.then(x.resolve).catch(x.reject)}var n=[...p.current.keys()].filter(b=>!p.current.get(b)[1]);if(!(!p.started||n.length===0)){var i=n.filter(b=>!this.current.has(b));if(i.length===0)e&&p.discard();else if(r.length>0){if(e)for(const b of this.#f)p.unskip_effect(b,y=>{(y.f&(At|br))!==0?p.schedule(y):p.#_([y])});p.activate();var o=new Set,c=new Map;for(var v of r)fn(v,i,o,c);c=new Map;var d=[...p.current].filter(([b,y])=>{const x=this.current.get(b);return x?x[0]!==y[0]||x[1]!==y[1]:!0}).map(([b])=>b);if(d.length>0)for(const b of this.#n)(b.f&(et|it|ns))===0&&sa(b,d,c)&&((b.f&(br|At))!==0?(Ye(b,Ve),p.schedule(b)):p.add_dirty_reaction(b,Ve));if(p.#t.length>0&&!p.#c){p.apply();for(var h of p.#u())p.#m(h,[],[])}p.deactivate()}}}}increment(e,r){if(this.#e+=1,e){let s=this.#i.get(r)??0;this.#i.set(r,s+1)}}decrement(e,r){if(this.#e-=1,e){let s=this.#i.get(r)??0;s===1?this.#i.delete(r):this.#i.set(r,s-1)}this.#c||(this.#c=!0,Ft(()=>{this.#c=!1,this.linked&&this.flush()}))}transfer_reactions(e){for(const[r,s]of e)this.add_dirty_reaction(r,s);e.clear()}add_dirty_reaction(e,r){(r===Ve||this.#a.get(e)!==Ve)&&this.#a.set(e,r)}oncommit(e){this.#r.add(e)}ondiscard(e){this.#o.add(e)}settled(){return(this.#s??=Ba()).promise}static ensure(){if(Se===null){const e=Se=new er;!Ps&&!Vr&&Ft(()=>{e.started||e.flush()})}return Se}apply(){{Et=null;return}}schedule(e){if(Ds=e,e.b?.is_pending&&(e.f&(yr|Yr|Js))!==0&&(e.f&$r)===0){e.b.defer_effect(e);return}this.#t.push(e)}#d(){if(this.linked){var e=this.prev,r=this.next;e===null?Ts=r:e.next=r,r===null?gr=e:r.prev=e,this.linked=!1}}}function ol(t){var e=Vr,r=mr;mr=null,Vr=!0;try{for(var s;;){if(Xi(),Se===null)return s;Se.flush()}}finally{Vr=e,mr=r}}function cl(){try{Qi()}catch(t){Rt(t,Ds)}}let Wt=null;function pa(t){var e=t.length;if(e!==0){for(var r=0;r<e;){var s=t[r++];if((s.f&(et|it))===0&&Zr(s)&&(Wt=new Set,Sr(s),s.deps===null&&s.first===null&&s.nodes===null&&s.teardown===null&&s.ac===null&&An(s),Wt?.size>0)){Vt.clear();for(const n of Wt){if((n.f&(et|it))!==0)continue;const i=[n];let o=n.parent;for(;o!==null;)Wt.has(o)&&(Wt.delete(o),i.push(o)),o=o.parent;for(let c=i.length-1;c>=0;c--){const v=i[c];(v.f&(et|it))===0&&Sr(v)}}Wt.clear()}}Wt=null}}function fn(t,e,r,s){if(!r.has(t)&&(r.add(t),t.reactions!==null))for(const n of t.reactions){const i=n.f;(i&st)!==0?fn(n,e,r,s):(i&(br|At))!==0&&(i&Ve)===0&&sa(n,e,s)&&(Ye(n,Ve),aa(n))}}function sa(t,e,r){const s=r.get(t);if(s!==void 0)return s;if(t.deps!==null)for(const n of t.deps){if(Wr.call(e,n))return!0;if((n.f&st)!==0&&sa(n,e,r))return r.set(n,!0),!0}return r.set(t,!1),!1}function aa(t){Se.schedule(t)}function hn(t,e){if(!((t.f&jt)!==0&&(t.f&Ze)!==0)){(t.f&Ve)!==0?e.d.push(t):(t.f&gt)!==0&&e.m.push(t),Ye(t,Ze);for(var r=t.first;r!==null;)hn(r,e),r=r.next}}function pn(t){Ye(t,Ze);for(var e=t.first;e!==null;)pn(e),e=e.next}let os=new Set;const Vt=new Map;let _n=!1;function tr(t,e){var r={f:0,v:t,reactions:null,equals:an,rv:0,wv:0};return r}function q(t,e){const r=tr(t);return Mn(r),r}function ul(t,e=!1,r=!0){const s=tr(t);return e||(s.equals=nn),Pr&&r&&He!==null&&He.l!==null&&(He.l.s??=[]).push(s),s}function g(t,e,r=!1){Ee!==null&&(!Mt||(Ee.f&ns)!==0)&&Jr()&&(Ee.f&(st|At|br|ns))!==0&&(Qt===null||!Qt.has(t))&&Gi();let s=r?Qe(e):e;return wr(t,s,as)}var or=null,Vs=0;function wr(t,e,r=null){if(!t.equals(e)){Bt?Vt.set(t,e):Vt.has(t)||Vt.set(t,t.v);var s=er.ensure();if(s.capture(t,e),(t.f&st)!==0){const n=t;(t.f&Ve)!==0&&ra(n),Et===null&&ea(n)}t.wv=jn(),or=null,Vs=0,gn(t,Ve,r),or=null,Jr()&&$e!==null&&($e.f&Ze)!==0&&($e.f&(jt|Zt))===0&&(mt===null?yl([t]):mt.push(t)),!s.is_fork&&os.size>0&&!_n&&vl()}return e}function vl(){_n=!1;for(const t of os){(t.f&Ze)!==0&&Ye(t,gt);let e;try{e=Zr(t)}catch{e=!0}e&&Sr(t)}os.clear()}function _a(t,e=1){var r=a(t),s=e===1?r++:r--;return g(t,r),s}function Qr(t){g(t,t.v+1)}function gn(t,e,r){var s=t.reactions;if(s!==null){var n=Jr(),i=s.length;if(Vs+=i,Vs>1e5&&or===null&&(or=new Set),or!==null){if(or.has(t))return;or.add(t)}for(var o=0;o<i;o++){var c=s[o],v=c.f;if(!(!n&&c===$e)){var d=(v&Ve)===0;if(d&&Ye(c,e),(v&ns)!==0)os.add(c);else if((v&st)!==0){var h=c;Et?.delete(h),gn(h,gt,r)}else if(d){var p=c;(v&At)!==0&&Wt!==null&&Wt.add(p),r!==null?r.push(p):aa(p)}}}}}function Qe(t){if(typeof t!="object"||t===null||Dt in t||Ya in t)return t;const e=Ks(t);if(e!==pi&&e!==_i)return t;var r=new Map,s=ds(t),n=q(0),i=fr,o=c=>{if(fr===i)return c();var v=Ee,d=fr;yt(null),ya(i);var h=c();return yt(v),ya(d),h};return s&&r.set("length",q(t.length)),new Proxy(t,{defineProperty(c,v,d){(!("value"in d)||d.configurable===!1||d.enumerable===!1||d.writable===!1)&&Wi();var h=r.get(v);return h===void 0?o(()=>{var p=q(d.value);return r.set(v,p),p}):g(h,d.value,!0),!0},deleteProperty(c,v){var d=r.get(v);if(d===void 0){if(v in c){const h=o(()=>q(Je));r.set(v,h),Qr(n)}}else g(d,Je),Qr(n);return!0},get(c,v,d){if(v===Dt)return t;var h=r.get(v),p=v in c;if(h===void 0&&(!p||Jt(c,v)?.writable)&&(h=o(()=>{var y=Qe(p?c[v]:Je),x=q(y);return x}),r.set(v,h)),h!==void 0){var b=a(h);return b===Je?void 0:b}return Reflect.get(c,v,d)},getOwnPropertyDescriptor(c,v){this.has?.(c,v);var d=Reflect.getOwnPropertyDescriptor(c,v),h=r.get(v);if(h!==void 0){var p=a(h);if(p===Je)return;if(d&&"value"in d)d.value=p;else return{enumerable:!0,configurable:!0,value:p,writable:!0}}return d},has(c,v){if(v===Dt)return!0;var d=r.get(v),h=d!==void 0&&d.v!==Je||Reflect.has(c,v);if(d!==void 0||($e!==null||Ee!==null)&&(!h||Jt(c,v)?.writable)){d===void 0&&(d=o(()=>{var b=h?Qe(c[v]):Je,y=q(b);return y}),r.set(v,d));var p=a(d);if(p===Je)return!1}return h},set(c,v,d,h){var p=r.get(v),b=v in c;if(s&&v==="length")for(var y=d;y<p.v;y+=1){var x=r.get(y+"");x!==void 0?g(x,Je):y in c&&(x=o(()=>q(Je)),r.set(y+"",x))}if(p===void 0)(!b||Jt(c,v)?.writable)&&(p=o(()=>q(void 0)),g(p,Qe(d)),r.set(v,p));else{b=p.v!==Je;var A=o(()=>Qe(d));g(p,A)}var k=Reflect.getOwnPropertyDescriptor(c,v);if(k?.set&&k.set.call(h,d),!b){if(s&&typeof v=="string"){var P=r.get("length"),E=Number(v);Number.isInteger(E)&&E>=P.v&&g(P,E+1)}Qr(n)}return!0},ownKeys(c){a(n);var v=Reflect.ownKeys(c).filter(p=>{var b=r.get(p);return b===void 0||b.v!==Je});for(var[d,h]of r)h.v!==Je&&!(d in c)&&v.push(d);return v},setPrototypeOf(){Bi()}})}function ga(t){try{if(t!==null&&typeof t=="object"&&Dt in t)return t[Dt]}catch{}return t}function bn(t,e){return Object.is(ga(t),ga(e))}var ba,mn,yn,xn;function dl(){if(ba===void 0){ba=window,mn=/Firefox/.test(navigator.userAgent);var t=Element.prototype,e=Node.prototype,r=Text.prototype;yn=Jt(e,"firstChild").get,xn=Jt(e,"nextSibling").get,da(t)&&(t[Cs]=void 0,t[Ja]=null,t[Os]=void 0,t.__e=void 0),da(r)&&(r[Fs]=void 0)}}function Ht(t=""){return document.createTextNode(t)}function It(t){return yn.call(t)}function Xr(t){return xn.call(t)}function f(t,e){return It(t)}function ke(t,e=!1){{var r=It(t);return r instanceof Comment&&r.data===""?Xr(r):r}}function $(t,e=!1){return It(t)}function u(t,e=1,r=!1){let s=t;for(;e--;)s=Xr(s);return s}function fl(t){t.textContent=""}function kn(){return!1}function ps(t,e,r){return e==null||e===tn?r?document.createElement(t,{is:r}):document.createElement(t):r?document.createElementNS(e,t,{is:r}):document.createElementNS(e,t)}function hl(t){var e=$e;if(e===null)return Ee.f|=Xt,t;if((e.f&$r)===0&&(e.f&yr)===0)throw t;Rt(t,e)}function Rt(t,e){if(t===en)throw t;if(!(e!==null&&(e.f&et)!==0)){for(;e!==null;){if((e.f&Rs)!==0&&(e.f&(et|Br))===0){if((e.f&$r)===0)throw t;try{e.b.error(t);return}catch(r){t=r}}e=e.parent}throw t}}function wn(t){$e===null&&(Ee===null&&Hi(),Vi()),Bt&&Di()}function pl(t,e){var r=e.last;r===null?e.last=e.first=t:(r.next=t,t.prev=r,e.last=t)}function qt(t,e){var r=$e;r!==null&&(r.f&it)!==0&&(t|=it);var s={ctx:He,deps:null,nodes:null,f:t|Ve|Nt,first:null,fn:e,last:null,next:null,parent:r,b:r&&r.b,prev:null,teardown:null,wv:0,ac:null};Se?.register_created_effect(s);var n=s;if((t&yr)!==0)Hr!==null?Hr.push(s):er.ensure().schedule(s);else if(e!==null){try{Sr(s)}catch(o){throw tt(s),o}n.deps===null&&n.teardown===null&&n.nodes===null&&n.first===n.last&&(n.f&Tr)===0&&(n=n.first,(t&At)!==0&&(t&hr)!==0&&n!==null&&(n.f|=hr))}if(n!==null&&(n.parent=r,r!==null&&pl(n,r),Ee!==null&&(Ee.f&st)!==0&&(t&Zt)===0)){var i=Ee;(i.effects??=[]).push(n)}return s}function na(){return Ee!==null&&!Mt}function _s(t){const e=qt(Yr,null);return Ye(e,Ze),e.teardown=t,e}function Ut(t){wn();var e=$e.f,r=!Ee&&(e&jt)!==0&&He!==null&&!He.i;if(r){var s=He;(s.e??=[]).push(t)}else return Sn(t)}function Sn(t){return qt(yr|Ga,t)}function _l(t){return wn(),qt(Yr|Ga,t)}function gl(t){er.ensure();const e=qt(Zt|Tr,t);return(r={})=>new Promise(s=>{r.outro?dr(e,()=>{tt(e),s(void 0)}):(tt(e),s(void 0))})}function ia(t){return qt(yr,t)}function bl(t){return qt(br|Tr,t)}function la(t,e=0){return qt(Yr|e,t)}function L(t,e=[],r=[],s=[]){cn(s,e,r,n=>{qt(Yr,()=>{t(...n.map(a))})})}function gs(t,e=0){var r=qt(At|e,t);return r}function $n(t,e=0){var r=qt(Js|e,t);return r}function ct(t){return qt(jt|Tr,t)}function Tn(t){var e=t.teardown;if(e!==null){const r=Bt,s=Ee;ma(!0),yt(null);try{e.call(null)}catch(n){Rt(n,t.parent)}finally{ma(r),yt(s)}}}function oa(t,e=!1){var r=t.first;for(t.first=t.last=null;r!==null;){const n=r.ac;n!==null&&Ar(()=>{n.abort(Kr)});var s=r.next;(r.f&Zt)!==0?r.parent=null:tt(r,e),r=s}}function ml(t){for(var e=t.first;e!==null;){var r=e.next;(e.f&jt)===0&&tt(e),e=r}}function tt(t,e=!0){var r=!1;(e||(t.f&yi)!==0)&&t.nodes!==null&&t.nodes.end!==null&&(Pn(t.nodes.start,t.nodes.end),r=!0),t.f|=Br,oa(t,e&&!r),Gr(t,0);var s=t.nodes&&t.nodes.t;if(s!==null)for(const i of s)i.stop();Tn(t),t.f^=Br,t.f|=et;var n=t.parent;n!==null&&n.first!==null&&An(t),t.next=t.prev=t.teardown=t.ctx=t.deps=t.fn=t.nodes=t.ac=t.b=null}function Pn(t,e){for(;t!==null;){var r=t===e?null:Xr(t);t.remove(),t=r}}function An(t){var e=t.parent,r=t.prev,s=t.next;r!==null&&(r.next=s),s!==null&&(s.prev=r),e!==null&&(e.first===t&&(e.first=s),e.last===t&&(e.last=r))}function dr(t,e,r=!0){var s=[];t.f|=Xs,En(t,s,!0);var n=()=>{r&&tt(t),e&&e()},i=s.length;if(i>0){var o=()=>--i||n();for(var c of s)c.out(o)}else n()}function En(t,e,r){if((t.f&it)===0){t.f^=it;var s=t.nodes&&t.nodes.t;if(s!==null)for(const c of s)(c.is_global||r)&&e.push(c);for(var n=t.first;n!==null;){var i=n.next;if((n.f&Zt)===0){var o=(n.f&hr)!==0||(n.f&jt)!==0&&(t.f&At)!==0;En(n,e,o?r:!1)}n=i}}}function cs(t){t.f&=~Xs,In(t,!0)}function In(t,e){if((t.f&Xs)===0&&(t.f&it)!==0){t.f^=it,(t.f&Ze)===0&&(Ye(t,Ve),er.ensure().schedule(t));for(var r=t.first;r!==null;){var s=r.next,n=(r.f&hr)!==0||(r.f&jt)!==0;In(r,n?e:!1),r=s}var i=t.nodes&&t.nodes.t;if(i!==null)for(const o of i)(o.is_global||e)&&o.in()}}function ca(t,e){if(t.nodes)for(var r=t.nodes.start,s=t.nodes.end;r!==null;){var n=r===s?null:Xr(r);e.append(r),r=n}}let Bt=!1;function ma(t){Bt=t}let Ee=null,Mt=!1;function yt(t){Ee=t}let $e=null;function xt(t){$e=t}let Qt=null;function Mn(t){Ee!==null&&((Ee.f&is)!==0||(Ee.f&st)!==0)&&(Qt??=new Set).add(t)}let vt=null,pt=0,mt=null;function yl(t){mt=t}let Nn=1,ur=0,fr=ur;function ya(t){fr=t}function jn(){return++Nn}function Zr(t){var e=t.f;if((e&Ve)!==0)return!0;if((e&gt)!==0){for(var r=t.deps,s=r.length,n=0;n<s;n++){var i=r[n];if(Zr(i)&&vn(i),i.wv>t.wv)return!0}(e&Nt)!==0&&Et===null&&Ye(t,Ze)}return!1}function qn(t,e,r=!0){var s=t.reactions;if(s!==null&&!(Qt!==null&&Qt.has(t)))for(var n=0;n<s.length;n++){var i=s[n];(i.f&st)!==0?qn(i,e,!1):e===i&&(r?Ye(i,Ve):(i.f&Ze)!==0&&Ye(i,gt),aa(i))}}function zn(t){var e=vt,r=pt,s=mt,n=Ee,i=Qt,o=He,c=Mt,v=fr,d=t.f;vt=null,pt=0,mt=null,Ee=(d&(jt|Zt))===0?t:null,Qt=null,xr(t.ctx),Mt=!1,fr=++ur,t.ac!==null&&(Ar(()=>{t.ac.abort(Kr)}),t.ac=null);try{t.f|=is;var h=t.fn,p=h();t.f|=$r;var b=xa(t);if(Jr()&&mt!==null&&!Mt&&b!==null&&(t.f&(st|gt|Ve))===0)for(var y=0;y<mt.length;y++)qn(mt[y],t);if(n!==null&&n!==t){if(ur++,n.deps!==null)for(let x=0;x<r;x+=1)n.deps[x].rv=ur;if(e!==null)for(const x of e)x.rv=ur;mt!==null&&(s===null?s=mt:s.push(...mt))}return(t.f&Xt)!==0&&(t.f^=Xt),p}catch(x){return xa(t),hl(x)}finally{t.f^=is,vt=e,pt=r,mt=s,Ee=n,Qt=i,xr(o),Mt=c,fr=v}}function xa(t){var e=t.deps,r=Se?.is_fork;if(vt!==null){var s;if(r||Gr(t,pt),e!==null&&pt>0)for(e.length=pt+vt.length,s=0;s<vt.length;s++)e[pt+s]=vt[s];else t.deps=e=vt;if(na()&&(t.f&Nt)!==0)for(s=pt;s<e.length;s++)(e[s].reactions??=[]).push(t)}else!r&&e!==null&&pt<e.length&&(Gr(t,pt),e.length=pt);return e}function xl(t,e){let r=e.reactions;if(r!==null){var s=hi.call(r,t);if(s!==-1){var n=r.length-1;n===0?r=e.reactions=null:(r[s]=r[n],r.pop())}}if(r===null&&(e.f&st)!==0&&(vt===null||!Wr.call(vt,e))){var i=e;(i.f&Nt)!==0&&(i.f^=Nt),i.v!==Je&&ea(i),i.ac!==null&&Ar(()=>{i.ac.abort(Kr),i.ac=null,Ye(i,Ve)}),il(i),Gr(i,0)}}function Gr(t,e){var r=t.deps;if(r!==null)for(var s=e;s<r.length;s++)xl(t,r[s])}function Sr(t){var e=t.f;if((e&et)===0){Ye(t,Ze);var r=$e;$e=t;try{(e&(At|Js))!==0?ml(t):oa(t),Tn(t);var s=zn(t);t.teardown=typeof s=="function"?s:null,t.wv=Nn;var n;fi&&Ki&&(t.f&Ve)!==0&&t.deps}finally{$e=r}}}async function kl(){await Promise.resolve(),ol()}function a(t){var e=t.f,r=(e&st)!==0;if(Ee!==null&&!Mt){var s=$e!==null&&($e.f&et)!==0;if(!s&&(Qt===null||!Qt.has(t))){var n=Ee.deps;if((Ee.f&is)!==0)t.rv<ur&&(t.rv=ur,vt===null&&n!==null&&n[pt]===t?pt++:vt===null?vt=[t]:vt.push(t));else{Ee.deps??=[],Wr.call(Ee.deps,t)||Ee.deps.push(t);var i=t.reactions;i===null?t.reactions=[Ee]:Wr.call(i,Ee)||i.push(Ee)}}}if(Bt&&Vt.has(t))return Vt.get(t);if(r){var o=t;if(Bt){var c=o.v;return((o.f&Ze)===0&&o.reactions!==null||Rn(o))&&(c=ra(o)),Vt.set(o,c),c}var v=(o.f&Nt)===0&&!Mt&&Ee!==null&&(Ee.f&Nt)!==0,d=(o.f&$r)===0;Zr(o)&&(v&&(o.f|=Nt),vn(o)),v&&!d&&(dn(o),Ln(o))}if(Et?.has(t))return Et.get(t);if((t.f&Xt)!==0)throw t.v;return t.v}function Ln(t){if(t.f|=Nt,t.deps!==null)for(const r of t.deps){var e=r.reactions;e===null?r.reactions=[t]:Wr.call(e,t)||e.push(t),(r.f&st)!==0&&(r.f&Nt)===0&&(dn(r),Ln(r))}}function Rn(t){if(t.v===Je)return!0;if(t.deps===null)return!1;for(const e of t.deps)if(Vt.has(e)||(e.f&st)!==0&&Rn(e))return!0;return!1}function rr(t){var e=Mt;try{return Mt=!0,t()}finally{Mt=e}}function lr(t){if(!(typeof t!="object"||!t||t instanceof EventTarget)){if(Dt in t)Hs(t);else if(!Array.isArray(t))for(let e in t){const r=t[e];typeof r=="object"&&r&&Dt in r&&Hs(r)}}}function Hs(t,e=new Set){if(typeof t=="object"&&t!==null&&!(t instanceof EventTarget)&&!e.has(t)){e.add(t),t instanceof Date&&t.getTime();for(let s in t)try{Hs(t[s],e)}catch{}const r=Ks(t);if(r!==Object.prototype&&r!==Array.prototype&&r!==Map.prototype&&r!==Set.prototype&&r!==Date.prototype){const s=Wa(r);for(let n in s){const i=s[n].get;if(i)try{i.call(t)}catch{}}}}}function wl(t){return t.endsWith("capture")&&t!=="gotpointercapture"&&t!=="lostpointercapture"}const Sl=["beforeinput","click","change","dblclick","contextmenu","focusin","focusout","input","keydown","keyup","mousedown","mousemove","mouseout","mouseover","mouseup","pointerdown","pointermove","pointerout","pointerover","pointerup","touchend","touchmove","touchstart"];function $l(t){return Sl.includes(t)}const Tl={formnovalidate:"formNoValidate",ismap:"isMap",nomodule:"noModule",playsinline:"playsInline",readonly:"readOnly",defaultvalue:"defaultValue",defaultchecked:"defaultChecked",srcobject:"srcObject",novalidate:"noValidate",allowfullscreen:"allowFullscreen",disablepictureinpicture:"disablePictureInPicture",disableremoteplayback:"disableRemotePlayback"};function Pl(t){return t=t.toLowerCase(),Tl[t]??t}const Al=["touchstart","touchmove"];function El(t){return Al.includes(t)}const Fr=Symbol("events"),Cn=new Set,Qs=new Set;function On(t,e,r,s={}){function n(i){if(s.capture||Us.call(e,i),!i.cancelBubble)return Ar(()=>r?.call(this,i))}return t.startsWith("pointer")||t.startsWith("touch")||t==="wheel"?(n.__removed=!1,Ft(()=>{n.__removed||e.addEventListener(t,n,s)})):e.addEventListener(t,n,s),n}function Ct(t,e,r,s,n){var i={capture:s,passive:n},o=On(t,e,r,i);(e===document.body||e===window||e===document||e instanceof HTMLMediaElement)&&_s(()=>{o.__removed=!0,e.removeEventListener(t,o,i)})}function m(t,e,r){(e[Fr]??={})[t]=r}function kt(t){for(var e=0;e<t.length;e++)Cn.add(t[e]);for(var r of Qs)r(t)}let As=null,Es=!1;function Us(t){var e=this,r=e.ownerDocument,s=t.type,n=t.composedPath?.()||[],i=n[0]||t.target;As=t,Es||(Es=!0,setTimeout(()=>{Es=!1,As=null}));var o=0,c=As===t&&t[Fr];if(c){var v=n.indexOf(c);if(v!==-1&&(e===document||e===window)){t[Fr]=e;return}var d=n.indexOf(e);if(d===-1)return;v<=d&&(o=v)}if(i=n[o]||t.target,i!==e){Ua(t,"currentTarget",{configurable:!0,get(){return i||r}});var h=Ee,p=$e;yt(null),xt(null);try{for(var b,y=[];i!==null&&i!==e;){try{var x=i[Fr]?.[s];x!=null&&(!i.disabled||t.target===i)&&x.call(i,t)}catch(A){b?y.push(A):b=A}if(t.cancelBubble)break;o++,i=o<n.length?n[o]:null}if(b){for(let A of y)queueMicrotask(()=>{throw A});throw b}}finally{t[Fr]=e,delete t.currentTarget,yt(h),xt(p)}}}const Il=globalThis?.window?.trustedTypes&&globalThis.window.trustedTypes.createPolicy("svelte-trusted-html",{createHTML:t=>t});function Ml(t){return Il?.createHTML(t)??t}function Fn(t){var e=ps("template");return e.innerHTML=Ml(t.replaceAll("<!>","<!---->")),e.content}function sr(t,e){var r=$e;r.nodes===null&&(r.nodes={start:t,end:e,a:null,t:null})}function w(t,e){var r=(e&Ii)!==0,s=(e&Mi)!==0,n,i=!t.startsWith("<!>");return()=>{n===void 0&&(n=Fn(i?t:"<!>"+t),r||(n=It(n)));var o=s||mn?document.importNode(n,!0):n.cloneNode(!0);if(r){var c=It(o),v=o.lastChild;sr(c,v)}else sr(o,o);return o}}function Nl(t,e,r="svg"){var s=!t.startsWith("<!>"),n=`<${r}>${s?t:"<!>"+t}</${r}>`,i;return()=>{if(!i){var o=Fn(n),c=It(o);i=It(c)}var v=i.cloneNode(!0);return sr(v,v),v}}function jl(t,e){return Nl(t,e,"svg")}function vr(t=""){{var e=Ht(t+"");return sr(e,e),e}}function Ne(){var t=document.createDocumentFragment(),e=document.createComment(""),r=Ht();return t.append(e,r),sr(e,r),t}function _(t,e){t!==null&&t.before(e)}function ql(t){let e=0,r=tr(0),s;return()=>{na()&&(a(r),la(()=>(e===0&&(s=rr(()=>t(()=>Qr(r)))),e+=1,()=>{Ft(()=>{e-=1,e===0&&(s?.(),s=void 0,Qr(r))})})))}}var zl=hr|Tr;function Ll(t,e,r,s){new Rl(t,e,r,s)}class Rl{parent;is_pending=!1;transform_error;#r;#o=null;#e;#i;#s;#t=null;#n=null;#a=null;#l=null;#f=0;#c=0;#h=!1;#g=new Map;#u=null;#p=ql(()=>(this.#u=tr(this.#f),()=>{this.#u=null}));constructor(e,r,s,n){this.#r=e,this.#e=r,this.#i=i=>{var o=$e;o.b=this,o.f|=Rs,s(i)},this.parent=$e.b,this.transform_error=n??this.parent?.transform_error??(i=>i),this.#s=gs(()=>{this.#y()},zl)}#m(){try{this.#t=ct(()=>this.#i(this.#r))}catch(e){this.error(e)}}#k(e){const r=this.#e.failed,{reset:s,invoke_onerror:n}=this.#b(e);Ft(n),r&&(this.#a=ct(()=>{r(this.#r,()=>e,()=>s)}))}#b(e){var r=!1,s=!1;const n=()=>{if(!this.#v()){if(r){Li();return}r=!0,s&&Yi(),this.#a!==null&&dr(this.#a,()=>{this.#a=null}),this.#x(()=>{this.#y()})}};return{reset:n,invoke_onerror:()=>{if(!this.#v())try{s=!0,this.#e.onerror?.(e,n),s=!1}catch(o){Rt(o,this.#s&&this.#s.parent)}}}}#v(){return(this.#s.f&(et|Br))!==0}#_(){const e=this.#e.pending;e&&(this.is_pending=!0,this.#n=ct(()=>e(this.#r)),Ft(()=>{if(!this.#v()){var r=this.#l=document.createDocumentFragment(),s=Ht(),n=!1;if(r.append(s),this.#t=this.#x(()=>{try{return ct(()=>this.#i(s))}catch(i){try{this.error(i),n=!0}catch(o){Rt(o,this.#s.parent)}return null}}),this.#t===null){this.#l=null,n&&this.#d(Se);return}this.#c===0&&(this.#r.before(r),this.#l=null,dr(this.#n,()=>{this.#n=null}),this.#d(Se))}}))}#y(){try{if(this.is_pending=this.has_pending_snippet(),this.#c=0,this.#f=0,this.#t=ct(()=>{this.#i(this.#r)}),this.#c>0){var e=this.#l=document.createDocumentFragment();ca(this.#t,e);const r=this.#e.pending;this.#n=ct(()=>r(this.#r))}else this.#d(Se)}catch(r){this.error(r)}}#d(e){this.is_pending=!1,e.transfer_reactions(this.#g)}defer_effect(e){on(e,this.#g)}is_rendered(){return!this.is_pending&&(!this.parent||this.parent.is_rendered())}has_pending_snippet(){return!!this.#e.pending}#x(e){var r=$e,s=Ee,n=He;xt(this.#s),yt(this.#s),xr(this.#s.ctx);try{return er.ensure(),e()}finally{xt(r),yt(s),xr(n)}}#w(e,r){if(!this.has_pending_snippet()){this.parent&&this.parent.#w(e,r);return}this.#c+=e,this.#c===0&&(this.#d(r),this.#n&&dr(this.#n,()=>{this.#n=null}),this.#l&&(this.#r.before(this.#l),this.#l=null))}update_pending_count(e,r){this.#w(e,r),this.#f+=e,!(!this.#u||this.#h)&&(this.#h=!0,Ft(()=>{this.#h=!1,this.#u&&wr(this.#u,this.#f)}))}get_effect_pending(){return this.#p(),a(this.#u)}error(e){if(e===en||!this.#e.onerror&&!this.#e.failed)throw e;Se?.is_fork?(this.#t&&Se.skip_effect(this.#t),this.#n&&Se.skip_effect(this.#n),this.#a&&Se.skip_effect(this.#a),Se.oncommit(()=>{this.#v()||this.#S(e)})):this.#S(e)}#S(e){this.#t&&(tt(this.#t),this.#t=null),this.#n&&(tt(this.#n),this.#n=null),this.#a&&(tt(this.#a),this.#a=null);let r=this.#e.failed;const s=n=>{if(this.#v())return;const{reset:i,invoke_onerror:o}=this.#b(n);o(),r&&!this.#v()&&(this.#a=this.#x(()=>{try{return ct(()=>{var c=$e;c.b=this,c.f|=Rs,r(this.#r,()=>n,()=>i)})}catch(c){return Rt(c,this.#s.parent),null}}))};Ft(()=>{if(!this.#v()){var n;try{n=this.transform_error(e)}catch(i){Rt(i,this.#s&&this.#s.parent);return}n!==null&&typeof n=="object"&&typeof n.then=="function"?n.then(s,i=>Rt(i,this.#s&&this.#s.parent)):s(n)}})}}function S(t,e){var r=e==null?"":typeof e=="object"?`${e}`:e;r!==(t[Fs]??=t.nodeValue)&&(t[Fs]=r,t.nodeValue=`${r}`)}function Cl(t,e){return Ol(t,e)}const ts=new Map;function Ol(t,{target:e,anchor:r,props:s={},events:n,context:i,intro:o=!0,transformError:c}){dl();var v=void 0,d=gl(()=>{var h=r??e.appendChild(Ht());Ll(h,{pending:()=>{}},y=>{ft({});var x=He;i&&(x.c=i),n&&(s.$$events=n),v=t(y,s)||Zs(),ht()},c);var p=new Set,b=y=>{for(var x=0;x<y.length;x++){var A=y[x];if(!p.has(A)){p.add(A);var k=El(A);for(const W of[e,document]){var P=ts.get(W);P===void 0&&(P=new Map,ts.set(W,P));var E=P.get(A);E===void 0?(W.addEventListener(A,Us,{passive:k}),P.set(A,1)):P.set(A,E+1)}}}};return b(fs(Cn)),Qs.add(b),()=>{for(var y of p)for(const k of[e,document]){var x=ts.get(k),A=x.get(y);--A==0?(k.removeEventListener(y,Us),x.delete(y),x.size===0&&ts.delete(k)):x.set(y,A)}Qs.delete(b),h!==r&&h.parentNode?.removeChild(h)}});return Fl.set(v,d),v}let Fl=new WeakMap;class Dn{anchor;#r=new Map;#o=new Map;#e=new Map;#i=new Set;#s=!0;constructor(e,r=!0){this.anchor=e,this.#s=r}#t=e=>{if(this.#r.has(e)){var r=this.#r.get(e),s=this.#o.get(r);if(s)cs(s),this.#i.delete(r);else{var n=this.#e.get(r);n&&(cs(n.effect),this.#o.set(r,n.effect),this.#e.delete(r),n.fragment.lastChild.remove(),this.anchor.before(n.fragment),s=n.effect)}for(const[i,o]of this.#r){if(this.#r.delete(i),i===e)break;const c=this.#e.get(o);c&&(tt(c.effect),this.#e.delete(o))}for(const[i,o]of this.#o){if(i===r||this.#i.has(i))continue;const c=()=>{if(Array.from(this.#r.values()).includes(i)){var d=document.createDocumentFragment();ca(o,d),d.append(Ht()),this.#e.set(i,{effect:o,fragment:d})}else tt(o);this.#i.delete(i),this.#o.delete(i)};this.#s||!s?(this.#i.add(i),dr(o,c,!1)):c()}}};#n=e=>{this.#r.delete(e);const r=Array.from(this.#r.values());for(const[s,n]of this.#e)r.includes(s)||(tt(n.effect),this.#e.delete(s))};ensure(e,r){var s=Se,n=kn();if(r&&!this.#o.has(e)&&!this.#e.has(e))if(n){var i=document.createDocumentFragment(),o=Ht();i.append(o),this.#e.set(e,{effect:ct(()=>r(o)),fragment:i})}else this.#o.set(e,ct(()=>r(this.anchor)));if(this.#r.set(s,e),n){for(const[c,v]of this.#o)c===e?s.unskip_effect(v):s.skip_effect(v);for(const[c,v]of this.#e)c===e?s.unskip_effect(v.effect):s.skip_effect(v.effect);s.oncommit(this.#t),s.ondiscard(this.#n)}else this.#t(s)}}function z(t,e,r=!1){var s=new Dn(t),n=r?hr:0;function i(o,c){s.ensure(o,c)}gs(()=>{var o=!1;e((c,v=0)=>{o=!0,i(v,c)}),o||i(-1,null)},n)}function pr(t,e){return e}function Dl(t,e,r){for(var s=[],n=e.length,i,o=e.length,c=0;c<n;c++){let p=e[c];dr(p,()=>{if(i){if(i.pending.delete(p),i.done.add(p),i.pending.size===0){var b=t.outrogroups;Ws(t,fs(i.done)),b.delete(i),b.size===0&&(t.outrogroups=null)}}else o-=1},!1)}if(o===0){var v=s.length===0&&r!==null&&t.pending.size===0;if(v){var d=r,h=d.parentNode;fl(h),h.append(d),t.items.clear()}Ws(t,e,!v)}else i={pending:new Set(e),done:new Set},(t.outrogroups??=new Set).add(i)}function Ws(t,e,r=!0){var s;if(t.pending.size>0){s=new Set;for(const o of t.pending.values())for(const c of o)s.add(t.items.get(c).e)}for(var n=0;n<e.length;n++){var i=e[n];if(s?.has(i)){i.f|=Ot;const o=document.createDocumentFragment();ca(i,o)}else tt(e[n],r)}}var ka;function Oe(t,e,r,s,n,i=null){var o=t,c=new Map,v=(e&Xa)!==0;if(v){var d=t;o=d.appendChild(Ht())}var h=null,p=ta(()=>{var E=r();return ds(E)?E:E==null?[]:fs(E)}),b=new Map,y=!0;function x(E){if((P.effect.f&et)===0){P.pending.delete(E);var W=a(p);P.fallback=h,Vl(P,W,o,e,s),h!==null&&(W.length===0?(h.f&Ot)===0?cs(h):(h.f^=Ot,Dr(h,null,o)):dr(h,()=>{h=null}))}}function A(E){P.pending.delete(E)}var k=gs(()=>{for(var E=a(p),W=E.length,K=new Set,he=Se,I=kn(),M=0;M<W;M+=1){var Z=E[M],ue=s(Z,M),j=y?null:c.get(ue);j?(j.v&&wr(j.v,Z),j.i&&wr(j.i,M),I&&he.unskip_effect(j.e)):(j=Hl(c,y?o:ka??=Ht(),Z,ue,M,n,e,r),y||(j.e.f|=Ot),c.set(ue,j)),K.add(ue)}if(W===0&&i&&!h&&(y?h=ct(()=>i(o)):(h=ct(()=>i(ka??=Ht())),h.f|=Ot)),W>K.size&&Fi(),!y)if(b.set(he,K),I){for(const[ie,N]of c)K.has(ie)||he.skip_effect(N.e);he.oncommit(x),he.ondiscard(A)}else x(he);a(p)}),P={effect:k,items:c,pending:b,outrogroups:null,fallback:h};y=!1}function Lr(t){for(;t!==null&&(t.f&jt)===0;)t=t.next;return t}function Vl(t,e,r,s,n){var i=(s&Si)!==0,o=e.length,c=t.items,v=Lr(t.effect.first),d,h=null,p,b=[],y=[],x,A,k,P;if(i)for(P=0;P<o;P+=1)x=e[P],A=n(x,P),k=c.get(A).e,(k.f&Ot)===0&&(k.nodes?.a?.measure(),(p??=new Set).add(k));for(P=0;P<o;P+=1){if(x=e[P],A=n(x,P),k=c.get(A).e,t.outrogroups!==null)for(const j of t.outrogroups)j.pending.delete(k),j.done.delete(k);if((k.f&it)!==0&&(cs(k),i&&(k.nodes?.a?.unfix(),(p??=new Set).delete(k))),(k.f&Ot)!==0)if(k.f^=Ot,k===v)Dr(k,null,r);else{var E=h?h.next:v;k===t.effect.last&&(t.effect.last=k.prev),k.prev&&(k.prev.next=k.next),k.next&&(k.next.prev=k.prev),Kt(t,h,k),Kt(t,k,E),Dr(k,E,r),h=k,b=[],y=[],v=Lr(h.next);continue}if(k!==v){if(d!==void 0&&d.has(k)){if(b.length<y.length){var W=y[0],K;h=W.prev;var he=b[0],I=b[b.length-1];for(K=0;K<b.length;K+=1)Dr(b[K],W,r);for(K=0;K<y.length;K+=1)d.delete(y[K]);Kt(t,he.prev,I.next),Kt(t,h,he),Kt(t,I,W),v=W,h=I,P-=1,b=[],y=[]}else d.delete(k),Dr(k,v,r),Kt(t,k.prev,k.next),Kt(t,k,h===null?t.effect.first:h.next),Kt(t,h,k),h=k;continue}for(b=[],y=[];v!==null&&v!==k;)(d??=new Set).add(v),y.push(v),v=Lr(v.next);if(v===null)continue}(k.f&Ot)===0&&b.push(k),h=k,v=Lr(k.next)}if(t.outrogroups!==null){for(const j of t.outrogroups)j.pending.size===0&&(Ws(t,fs(j.done)),t.outrogroups?.delete(j));t.outrogroups.size===0&&(t.outrogroups=null)}if(v!==null||d!==void 0){var M=[];if(d!==void 0)for(k of d)(k.f&it)===0&&M.push(k);for(;v!==null;)(v.f&it)===0&&v!==t.fallback&&M.push(v),v=Lr(v.next);var Z=M.length;if(Z>0){var ue=(s&Xa)!==0&&o===0?r:null;if(i){for(P=0;P<Z;P+=1)M[P].nodes?.a?.measure();for(P=0;P<Z;P+=1)M[P].nodes?.a?.fix()}Dl(t,M,ue)}}i&&Ft(()=>{if(p!==void 0)for(k of p)k.nodes?.a?.apply()})}function Hl(t,e,r,s,n,i,o,c){var v=(o&ki)!==0?(o&$i)===0?ul(r,!1,!1):tr(r):null,d=(o&wi)!==0?tr(n):null;return{v,i:d,e:ct(()=>(i(e,v??r,d??n,c),()=>{t.delete(s)}))}}function Dr(t,e,r){if(t.nodes)for(var s=t.nodes.start,n=t.nodes.end,i=e&&(e.f&Ot)===0?e.nodes.start:r;s!==null;){var o=Xr(s);if(i.before(s),s===n)return;s=o}}function Kt(t,e,r){e===null?t.effect.first=r:e.next=r,r===null?t.effect.last=e:r.prev=e}function wa(t,e,r=!1,s=!1,n=!1,i=!1,o){var c=t,v="";if(r)var d=t;L(()=>{var h=$e;if(v!==(v=e()??"")){if(r&&!sn){h.nodes=null,d.innerHTML=v,v!==""&&sr(It(d),d.lastChild);return}if(h.nodes!==null&&(Pn(h.nodes.start,h.nodes.end),h.nodes=null),v!==""){var p=s?rn:n?Ni:void 0,b=ps(s?"svg":n?"math":"template",p);b.innerHTML=v;var y=s||n?b:b.content;if(sr(It(y),y.lastChild),s||n)for(;It(y);)c.before(It(y));else c.before(y)}}})}function Re(t,e,r,s,n,i){if(e.$$host?.$$shadowRoot){const v=ps("slot");_(t,v);return}var o=e.$$slots?.[r],c=!1;o===!0&&(o=e.children,c=!0),o===void 0||o(t,c?()=>s:s)}function Ql(t,e,r,s,n,i){var o=null,c=t,v=new Dn(c,!1);gs(()=>{const d=e()||null;var h=rn;if(d===null){v.ensure(null,null);return}return v.ensure(d,p=>{if(d){var b=sn;if(o=ps(d,h),sr(o,o),s){var y=null,x=o.appendChild(Ht());s(o,x),y?.remove()}$e.nodes.end=o,b||p.before(o)}}),()=>{}},hr),_s(()=>{})}function Ul(t,e){var r=void 0,s;$n(()=>{r!==(r=e())&&(s&&(tt(s),s=null),r&&(s=ct(()=>{ia(()=>r(t))})))})}function Vn(t){var e,r,s="";if(typeof t=="string"||typeof t=="number")s+=t;else if(typeof t=="object")if(Array.isArray(t)){var n=t.length;for(e=0;e<n;e++)t[e]&&(r=Vn(t[e]))&&(s&&(s+=" "),s+=r)}else for(r in t)t[r]&&(s&&(s+=" "),s+=r);return s}function Wl(){for(var t,e,r=0,s="",n=arguments.length;r<n;r++)(t=arguments[r])&&(e=Vn(t))&&(s&&(s+=" "),s+=e);return s}function Hn(t){return typeof t=="object"?Wl(t):t??""}const Sa=[...` 	
\r\f \v\uFEFF`];function Bl(t,e,r){var s=t==null?"":""+t;if(e&&(s=s?s+" "+e:e),r){for(var n of Object.keys(r))if(r[n])s=s?s+" "+n:n;else if(s.length)for(var i=n.length,o=0;(o=s.indexOf(n,o))>=0;){var c=o+i;(o===0||Sa.includes(s[o-1]))&&(c===s.length||Sa.includes(s[c]))?s=(o===0?"":s.substring(0,o))+s.substring(c+1):o=c}}return s===""?null:s}function $a(t,e=!1){var r=e?" !important;":";",s="";for(var n of Object.keys(t)){var i=t[n];i!=null&&i!==""&&(s+=" "+n+": "+i+r)}return s}function Is(t){return t[0]!=="-"||t[1]!=="-"?t.toLowerCase():t}function Gl(t,e){if(e){var r="",s,n;if(Array.isArray(e)?(s=e[0],n=e[1]):s=e,t){t=String(t).replaceAll(/\/\*.*?\*\//g,"").trim();var i=!1,o=0,c=!1,v=[];s&&v.push(...Object.keys(s).map(Is)),n&&v.push(...Object.keys(n).map(Is));var d=0,h=-1;const A=t.length;for(var p=0;p<A;p++){var b=t[p];if(c?b==="/"&&t[p-1]==="*"&&(c=!1):i?i===b&&(i=!1):b==="/"&&t[p+1]==="*"?c=!0:b==='"'||b==="'"?i=b:b==="("?o++:b===")"&&o--,!c&&i===!1&&o===0){if(b===":"&&h===-1)h=p;else if(b===";"||p===A-1){if(h!==-1){var y=Is(t.substring(d,h).trim());if(!v.includes(y)){b!==";"&&p++;var x=t.substring(d,p).trim();r+=" "+x+";"}}d=p+1,h=-1}}}}return s&&(r+=$a(s)),n&&(r+=$a(n,!0)),r=r.trim(),r===""?null:r}return t==null?null:String(t)}function re(t,e,r,s,n,i){var o=t[Cs];if(o!==r||o===void 0){var c=Bl(r,s,i);c==null?t.removeAttribute("class"):e?t.className=c:t.setAttribute("class",c),t[Cs]=r}else if(i&&n!==i)for(var v in i){var d=!!i[v];(n==null||d!==!!n[v])&&t.classList.toggle(v,d)}return i}function Ms(t,e={},r,s){for(var n in r){var i=r[n];e[n]!==i&&(r[n]==null?t.style.removeProperty(n):t.style.setProperty(n,i,s))}}function bt(t,e,r,s){var n=t[Os];if(n!==e){var i=Gl(e,s);i==null?t.removeAttribute("style"):t.style.cssText=i,t[Os]=e}else s&&(Array.isArray(s)?(Ms(t,r?.[0],s[0]),Ms(t,r?.[1],s[1],"important")):Ms(t,r,s));return s}function Qn(t,e){e?t.hasAttribute("selected")||t.setAttribute("selected",""):t.removeAttribute("selected")}function Ta(t,e){var r=!("__defaultValue"in t);!r&&t.__defaultValue===e||(t.__defaultValue=e,Un(t,!r||"__value"in t))}function Un(t,e){var r=t.__defaultValue,s=t.multiple,n=s?r??[]:null;if(!(s&&!ds(n))){var i=t.selectedIndex,o=e&&s?new Set(t.selectedOptions):null;for(var c of t.options){var v=Bs(c);Qn(c,s?n.includes(v):bn(v,r))}if(e)if(o!==null)for(c of t.options){var d=o.has(c);c.selected!==d&&(c.selected=d)}else t.selectedIndex!==i&&(t.selectedIndex=i)}}function us(t,e,r=!1){if(t.multiple){if(e==null)return;if(!ds(e))return zi();for(var s of t.options)s.selected=e.includes(Bs(s));return}for(s of t.options){var n=Bs(s);if(bn(n,e)){s.selected=!0;return}}(!r||e!==void 0)&&(t.selectedIndex=-1)}function Wn(t){var e=new MutationObserver(r=>{r.every(Yl)||("__defaultValue"in t&&Un(t,!1),"__value"in t&&us(t,t.__value))});e.observe(t,{childList:!0,subtree:!0,attributes:!0,attributeFilter:["value"]}),_s(()=>{e.disconnect()})}function Bs(t){return"__value"in t?t.__value:t.value}function Yl(t){if(t.target.closest("selectedcontent")!==null)return!0;if(t.type==="childList"){var e=[...t.addedNodes,...t.removedNodes];return e.length>0&&e.every(r=>r.nodeName==="SELECTEDCONTENT")}return!1}const Rr=Symbol("class"),Cr=Symbol("style"),Bn=Symbol("is custom element"),Gn=Symbol("is html"),Kl=hs?"input":"INPUT",Jl=hs?"option":"OPTION",Yn=hs?"select":"SELECT",Kn=hs?"progress":"PROGRESS";function Gs(t,e){var r=bs(t);r.value===(r.value=e??void 0)||t.value===e&&(e!==0||t.nodeName!==Kn)||(t.value=e??"")}function Ur(t,e){var r=bs(t);r.checked!==(r.checked=e??void 0)&&(t.checked=e)}function _t(t,e,r,s){var n=bs(t),i=n[e];i!==(n[e]=r)&&(e==="loading"&&(t[xi]=r),r==null?t.removeAttribute(e):typeof r!="string"&&Jn(t).has(e)?t[e]=r:t.setAttribute(e,r))}function Xl(t,e,r,s,n=!1,i=!1){var o=bs(t),c=o[Bn],v=!o[Gn],d=e||{},h=t.nodeName===Jl,p=t.nodeName===Yn;try{for(var b in e)!(b in r)&&b[0]+b[1]!=="$$"&&(r[b]=null);r.class?r.class=Hn(r.class):(s||r[Rr])&&(r.class=null),r[Cr]&&(r.style??=null);var y=Jn(t);if(t.nodeName===Kl&&"type"in r&&("value"in r||"__value"in r)){var x=r.type;(x!==d.type||x===void 0&&t.hasAttribute("type"))&&(d.type=x,_t(t,"type",x,i))}for(const I in r){let M=r[I];if(h&&I==="value"&&M==null){t.value=t.__value="",d[I]=M;continue}if(I==="class"){var A=t.namespaceURI==="http://www.w3.org/1999/xhtml";re(t,A,M,s,e?.[Rr],r[Rr]),d[I]=M,d[Rr]=r[Rr];continue}if(I==="style"){bt(t,M,e?.[Cr],r[Cr]),d[I]=M,d[Cr]=r[Cr];continue}var k=d[I];if(!(M===k&&!(M===void 0&&t.hasAttribute(I)))){d[I]=M;var P=I[0]+I[1];if(P!=="$$")if(P==="on"){const Z={},ue="$$"+I;let j=I.slice(2);var E=$l(j);if(wl(j)&&(j=j.slice(0,-7),Z.capture=!0),!E&&k){if(M!=null)continue;t.removeEventListener(j,d[ue],Z),d[ue]=null}if(E)m(j,t,M),kt([j]);else if(M!=null){let ie=function(N){d[I].call(this,N)};var he=ie;d[ue]=On(j,t,ie,Z)}}else if(I==="style")_t(t,I,M);else if(I==="autofocus")el(t,!!M);else if(!c&&(I==="__value"||I==="value"&&M!=null))t.__value=M,(k==null||t.value!==M||M===0&&t.nodeName===Kn)&&(t.value=M);else if(I==="selected"&&h)Qn(t,M);else{var W=I;v||(W=Pl(W));var K=W==="defaultValue"||W==="defaultChecked";if(p&&W==="defaultValue")continue;if(M==null&&!c&&!K)if(o[I]=null,W==="value"||W==="checked"){let Z=t;const ue=e===void 0;if(W==="value"){let j=Z.defaultValue;Z.removeAttribute(W),Z.defaultValue=j,Z.value=Z.__value=ue?j:null}else{let j=Z.defaultChecked;Z.removeAttribute(W),Z.defaultChecked=j,Z.checked=ue?j:!1}}else t.removeAttribute(I);else K||(c||typeof M!="string")&&y.has(W)?(t[W]=M,W in o&&(o[W]=Je)):typeof M!="function"&&_t(t,W,M,i)}}}}finally{}return d}function Pa(t,e,r=[],s=[],n=[],i,o=!1,c=!1){cn(n,r,s,v=>{var d=void 0,h={},p=t.nodeName===Yn,b=!1;if($n(()=>{var x=e(...v.map(a)),A=Xl(t,d,x,i,o,c);if(b&&p){var k=t;"defaultValue"in x&&Ta(k,x.defaultValue),"value"in x&&us(k,x.value)}for(let E of Object.getOwnPropertySymbols(h))x[E]||tt(h[E]);for(let E of Object.getOwnPropertySymbols(x)){var P=x[E];E.description===ji&&(!d||P!==d[E])&&(h[E]&&tt(h[E]),h[E]=ct(()=>Ul(t,()=>P))),A[E]=P}d=A}),p){var y=t;ia(()=>{var x=d;"defaultValue"in x&&Ta(y,x.defaultValue),us(y,x.value,!0),Wn(y)})}b=!0})}function bs(t){return t[Ja]??={[Bn]:t.nodeName.includes("-"),[Gn]:t.namespaceURI===tn}}var Aa=new Map;function Jn(t){var e=t.getAttribute("is")||t.nodeName,r=Aa.get(e);if(r)return r;Aa.set(e,r=new Set);for(var s,n=t,i=Element.prototype;i!==n;){s=Wa(n);for(var o in s)s[o].set&&o!=="innerHTML"&&o!=="textContent"&&o!=="innerText"&&r.add(o);n=Ks(n)}return r}function vs(t,e,r=e){var s=new WeakSet;rl(t,"input",async n=>{var i=n?t.defaultValue:t.value;if(i=Ns(t)?js(i):i,r(i),Se!==null&&s.add(Se),await kl(),i!==(i=e())){var o=t.selectionStart,c=t.selectionEnd,v=t.value.length;if(t.value=i??"",c!==null){var d=t.value.length;o===c&&c===v&&d>v?(t.selectionStart=d,t.selectionEnd=d):(t.selectionStart=o,t.selectionEnd=Math.min(c,d))}}}),rr(e)==null&&t.value&&(r(Ns(t)?js(t.value):t.value),Se!==null&&s.add(Se)),la(()=>{var n=e();if(t===document.activeElement){var i=Se;if(s.has(i))return}Ns(t)&&n===js(t.value)||t.type==="date"&&!n&&!t.value||n!==t.value&&(t.value=n??"")})}function Ns(t){var e=t.type;return e==="number"||e==="range"}function js(t){return t===""?null:+t}function qs(t,e){return t===e||t?.[Dt]===e}function ms(t=Zs(),e,r,s){var n=He.r,i=$e;return ia(()=>{var o,c;return la(()=>{o=c,c=s?.()||[],rr(()=>{qs(r(...c),t)||(e(t,...c),o&&qs(r(...o),t)&&e(null,...o))})}),()=>{let v=i;for(;v!==n&&v.parent!==null&&v.parent.f&Br;)v=v.parent;const d=()=>{c&&qs(r(...c),t)&&e(null,...c)},h=v.teardown;v.teardown=()=>{d(),h?.()}}}),t}function Zl(t=!1){const e=He,r=e.l.u;if(!r)return;let s=()=>lr(e.s);if(t){let n=0,i={};const o=kr(()=>{let c=!1;const v=e.s;for(const d in v)v[d]!==i[d]&&(i[d]=v[d],c=!0);return c&&n++,n});s=()=>a(o)}r.b.length&&_l(()=>{Ea(e,s),Ls(r.b)}),Ut(()=>{const n=rr(()=>r.m.map(bi));return()=>{for(const i of n)typeof i=="function"&&i()}}),r.a.length&&Ut(()=>{Ea(e,s),Ls(r.a)})}function Ea(t,e){if(t.l.s)for(const r of t.l.s)a(r);e()}let rs=!1;function eo(t){var e=rs;try{return rs=!1,[t(),rs]}finally{rs=e}}const to={get(t,e){if(!t.exclude.includes(e))return a(t.version),e in t.special?t.special[e]():t.props[e]},set(t,e,r){if(!(e in t.special)){var s=$e;try{xt(t.parent_effect),t.special[e]=ut({get[e](){return t.props[e]}},e,Za)}finally{xt(s)}}return t.special[e](r),_a(t.version),!0},getOwnPropertyDescriptor(t,e){if(!t.exclude.includes(e)&&e in t.props)return{enumerable:!0,configurable:!0,value:t.props[e]}},deleteProperty(t,e){return t.exclude.includes(e)||(t.exclude.push(e),_a(t.version)),!0},has(t,e){return t.exclude.includes(e)?!1:e in t.props},ownKeys(t){return Reflect.ownKeys(t.props).filter(e=>!t.exclude.includes(e))}};function ze(t,e){return new Proxy({props:t,exclude:e,special:{},version:tr(0),parent_effect:$e},to)}const ro={get(t,e){let r=t.props.length;for(;r--;){let s=t.props[r];if(zr(s)&&(s=s()),typeof s=="object"&&s!==null&&e in s)return s[e]}},set(t,e,r){let s=t.props.length;for(;s--;){let n=t.props[s];zr(n)&&(n=n());const i=Jt(n,e);if(i&&i.set)return i.set(r),!0}return!1},getOwnPropertyDescriptor(t,e){let r=t.props.length;for(;r--;){let s=t.props[r];if(zr(s)&&(s=s()),typeof s=="object"&&s!==null&&e in s){const n=Jt(s,e);return n&&!n.configurable&&(n.configurable=!0),n}}},has(t,e){if(e===Dt||e===Ka)return!1;for(let r of t.props)if(zr(r)&&(r=r()),r!=null&&e in r)return!0;return!1},ownKeys(t){const e=[];for(let r of t.props)if(zr(r)&&(r=r()),!!r){for(const s in r)e.includes(s)||e.push(s);for(const s of Object.getOwnPropertySymbols(r))e.includes(s)||e.push(s)}return e}};function Fe(...t){return new Proxy({props:t},ro)}function ut(t,e,r,s){var n=!Pr||(r&Pi)!==0,i=(r&Ai)!==0,o=(r&Ei)!==0,c=s,v=!0,d=void 0,h=()=>o&&n?(d??=kr(s),a(d)):(v&&(v=!1,c=o?rr(s):s),c);let p;if(i){var b=Dt in t||Ka in t;p=Jt(t,e)?.set??(b&&e in t?K=>t[e]=K:void 0)}var y,x=!1;i?[y,x]=eo(()=>t[e]):y=t[e],y===void 0&&s!==void 0&&(y=h(),p&&(n&&Ui(),p(y)));var A;if(n?A=()=>{var K=t[e];return K===void 0?h():(v=!0,K)}:A=()=>{var K=t[e];return K!==void 0&&(c=void 0),K===void 0?c:K},n&&(r&Za)===0)return A;if(p){var k=t.$$legacy;return(function(K,he){return arguments.length>0?((!n||!he||k||x)&&p(he?A():K),K):A()})}var P=!1,E=((r&Ti)!==0?kr:ta)(()=>(P=!1,A()));i&&a(E);var W=$e;return(function(K,he){if(arguments.length>0){const I=he?a(E):n&&i?Qe(K):K;return g(E,I),P=!0,c!==void 0&&(c=I),K}return Bt&&P||(W.f&et)!==0?E.v:a(E)})}function so(t){He===null&&Ci(),Pr&&He.l!==null?ao(He).m.push(t):Ut(()=>{const e=rr(t);if(typeof e=="function")return e})}function ao(t){var e=t.l;return e.u??={a:[],b:[],m:[]}}const no="5";typeof window<"u"&&((window.__svelte??={}).v??=new Set).add(no);Ji();/**
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
 */const io={xmlns:"http://www.w3.org/2000/svg",width:24,height:24,viewBox:"0 0 24 24",fill:"none",stroke:"currentColor","stroke-width":2,"stroke-linecap":"round","stroke-linejoin":"round"};/**
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
 */const lo=t=>{for(const e in t)if(e.startsWith("aria-")||e==="role"||e==="title")return!0;return!1};/**
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
 */const Ia=(...t)=>t.filter((e,r,s)=>!!e&&e.trim()!==""&&s.indexOf(e)===r).join(" ").trim();var oo=jl("<svg><!><!></svg>");function De(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]),s=ze(r,["name","color","size","strokeWidth","absoluteStrokeWidth","iconNode"]);ft(e,!1);let n=ut(e,"name",8,void 0),i=ut(e,"color",8,"currentColor"),o=ut(e,"size",8,24),c=ut(e,"strokeWidth",8,2),v=ut(e,"absoluteStrokeWidth",8,!1),d=ut(e,"iconNode",24,()=>[]);Zl();var h=oo();Pa(h,(y,x,A)=>({...io,...y,...s,width:o(),height:o(),stroke:i(),"stroke-width":x,class:A}),[()=>lo(s)?void 0:{"aria-hidden":"true"},()=>(lr(v()),lr(c()),lr(o()),rr(()=>v()?Number(c())*24/Number(o()):c())),()=>(lr(Ia),lr(n()),lr(r),rr(()=>Ia("lucide-icon","lucide",n()?`lucide-${n()}`:"",r.class)))]);var p=f(h);Oe(p,1,d,pr,(y,x)=>{var A=me(()=>mi(a(x),2));let k=()=>a(A)[0],P=()=>a(A)[1];var E=Ne(),W=ke(E);Ql(W,k,!0,(K,he)=>{Pa(K,()=>({...P()}))}),_(y,E)});var b=u(p);Re(b,e,"default",{}),_(t,h),ht()}function co(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"m15 18-6-6 6-6"}]];De(t,Fe({name:"chevron-left"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function uo(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["circle",{cx:"12",cy:"12",r:"10"}],["path",{d:"m16.24 7.76-1.804 5.411a2 2 0 0 1-1.265 1.265L7.76 16.24l1.804-5.411a2 2 0 0 1 1.265-1.265z"}]];De(t,Fe({name:"compass"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function vo(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["circle",{cx:"9",cy:"12",r:"1"}],["circle",{cx:"9",cy:"5",r:"1"}],["circle",{cx:"9",cy:"19",r:"1"}],["circle",{cx:"15",cy:"12",r:"1"}],["circle",{cx:"15",cy:"5",r:"1"}],["circle",{cx:"15",cy:"19",r:"1"}]];De(t,Fe({name:"grip-vertical"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function Xn(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M2 9.5a5.5 5.5 0 0 1 9.591-3.676.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5l-5.492 5.313a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5"}]];De(t,Fe({name:"heart"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function fo(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8"}],["path",{d:"M3 10a2 2 0 0 1 .709-1.528l7-6a2 2 0 0 1 2.582 0l7 6A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"}]];De(t,Fe({name:"house"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function ho(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"m16 6 4 14"}],["path",{d:"M12 6v14"}],["path",{d:"M8 8v12"}],["path",{d:"M4 4v16"}]];De(t,Fe({name:"library"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function Ma(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M16 5H3"}],["path",{d:"M11 12H3"}],["path",{d:"M11 19H3"}],["path",{d:"M21 16V5"}],["circle",{cx:"18",cy:"16",r:"3"}]];De(t,Fe({name:"list-music"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function po(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M14 14a2 2 0 0 0 2-2V8h-2"}],["path",{d:"M22 17a2 2 0 0 1-2 2H6.828a2 2 0 0 0-1.414.586l-2.202 2.202A.71.71 0 0 1 2 21.286V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z"}],["path",{d:"M8 14a2 2 0 0 0 2-2V8H8"}]];De(t,Fe({name:"message-square-quote"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function Na(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M9 18V5l12-2v13"}],["circle",{cx:"6",cy:"18",r:"3"}],["circle",{cx:"18",cy:"16",r:"3"}]];De(t,Fe({name:"music"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function _o(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["rect",{width:"18",height:"18",x:"3",y:"3",rx:"2"}],["path",{d:"M9 3v18"}]];De(t,Fe({name:"panel-left"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function Zn(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["rect",{x:"14",y:"3",width:"5",height:"18",rx:"1"}],["rect",{x:"5",y:"3",width:"5",height:"18",rx:"1"}]];De(t,Fe({name:"pause"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function ei(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M5 5a2 2 0 0 1 3.008-1.728l11.997 6.998a2 2 0 0 1 .003 3.458l-12 7A2 2 0 0 1 5 19z"}]];De(t,Fe({name:"play"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function go(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M5 12h14"}],["path",{d:"M12 5v14"}]];De(t,Fe({name:"plus"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function bo(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M16.247 7.761a6 6 0 0 1 0 8.478"}],["path",{d:"M19.075 4.933a10 10 0 0 1 0 14.134"}],["path",{d:"M4.925 19.067a10 10 0 0 1 0-14.134"}],["path",{d:"M7.753 16.239a6 6 0 0 1 0-8.478"}],["circle",{cx:"12",cy:"12",r:"2"}]];De(t,Fe({name:"radio"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function mo(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"m17 2 4 4-4 4"}],["path",{d:"M3 11v-1a4 4 0 0 1 4-4h14"}],["path",{d:"m7 22-4-4 4-4"}],["path",{d:"M21 13v1a4 4 0 0 1-4 4H3"}]];De(t,Fe({name:"repeat"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function ja(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"m21 21-4.34-4.34"}],["circle",{cx:"11",cy:"11",r:"8"}]];De(t,Fe({name:"search"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function yo(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M9.671 4.136a2.34 2.34 0 0 1 4.659 0 2.34 2.34 0 0 0 3.319 1.915 2.34 2.34 0 0 1 2.33 4.033 2.34 2.34 0 0 0 0 3.831 2.34 2.34 0 0 1-2.33 4.033 2.34 2.34 0 0 0-3.319 1.915 2.34 2.34 0 0 1-4.659 0 2.34 2.34 0 0 0-3.32-1.915 2.34 2.34 0 0 1-2.33-4.033 2.34 2.34 0 0 0 0-3.831A2.34 2.34 0 0 1 6.35 6.051a2.34 2.34 0 0 0 3.319-1.915"}],["circle",{cx:"12",cy:"12",r:"3"}]];De(t,Fe({name:"settings"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function xo(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"m18 14 4 4-4 4"}],["path",{d:"m18 2 4 4-4 4"}],["path",{d:"M2 18h1.973a4 4 0 0 0 3.3-1.7l5.454-8.6a4 4 0 0 1 3.3-1.7H22"}],["path",{d:"M2 6h1.972a4 4 0 0 1 3.6 2.2"}],["path",{d:"M22 18h-6.041a4 4 0 0 1-3.3-1.8l-.359-.45"}]];De(t,Fe({name:"shuffle"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function ti(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M17.971 4.285A2 2 0 0 1 21 6v12a2 2 0 0 1-3.029 1.715l-9.997-5.998a2 2 0 0 1-.003-3.432z"}],["path",{d:"M3 20V4"}]];De(t,Fe({name:"skip-back"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function ri(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M21 4v16"}],["path",{d:"M6.029 4.285A2 2 0 0 0 3 6v12a2 2 0 0 0 3.029 1.715l9.997-5.998a2 2 0 0 0 .003-3.432z"}]];De(t,Fe({name:"skip-forward"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function si(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M11.017 2.814a1 1 0 0 1 1.966 0l1.051 5.558a2 2 0 0 0 1.594 1.594l5.558 1.051a1 1 0 0 1 0 1.966l-5.558 1.051a2 2 0 0 0-1.594 1.594l-1.051 5.558a1 1 0 0 1-1.966 0l-1.051-5.558a2 2 0 0 0-1.594-1.594l-5.558-1.051a1 1 0 0 1 0-1.966l5.558-1.051a2 2 0 0 0 1.594-1.594z"}],["path",{d:"M20 2v4"}],["path",{d:"M22 4h-4"}],["circle",{cx:"4",cy:"20",r:"2"}]];De(t,Fe({name:"sparkles"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function ko(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["line",{x1:"10",x2:"14",y1:"2",y2:"2"}],["line",{x1:"12",x2:"15",y1:"14",y2:"11"}],["circle",{cx:"12",cy:"14",r:"8"}]];De(t,Fe({name:"timer"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function wo(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"}],["path",{d:"M16 3.128a4 4 0 0 1 0 7.744"}],["path",{d:"M22 21v-2a4 4 0 0 0-3-3.87"}],["circle",{cx:"9",cy:"7",r:"4"}]];De(t,Fe({name:"users"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function qa(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M11 4.702a.705.705 0 0 0-1.203-.498L6.413 7.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.705.705 0 0 0 11 19.298z"}],["path",{d:"M16 9a5 5 0 0 1 0 6"}]];De(t,Fe({name:"volume-1"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function za(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M11 4.702a.705.705 0 0 0-1.203-.498L6.413 7.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.705.705 0 0 0 11 19.298z"}],["path",{d:"M16 9a5 5 0 0 1 0 6"}],["path",{d:"M19.364 18.364a9 9 0 0 0 0-12.728"}]];De(t,Fe({name:"volume-2"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function La(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M11 4.702a.705.705 0 0 0-1.203-.498L6.413 7.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.705.705 0 0 0 11 19.298z"}],["line",{x1:"22",x2:"16",y1:"9",y2:"15"}],["line",{x1:"16",x2:"22",y1:"9",y2:"15"}]];De(t,Fe({name:"volume-x"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}function So(t,e){const r=ze(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M18 6 6 18"}],["path",{d:"m6 6 12 12"}]];De(t,Fe({name:"x"},()=>r,{get iconNode(){return s},children:(n,i)=>{var o=Ne(),c=ke(o);Re(c,e,"default",{}),_(n,o)},$$slots:{default:!0}}))}const $o="modulepreload",To=function(t){return"/"+t},Ra={},Po=function(e,r,s){let n=Promise.resolve();if(r&&r.length>0){let o=function(d){return Promise.all(d.map(h=>Promise.resolve(h).then(p=>({status:"fulfilled",value:p}),p=>({status:"rejected",reason:p}))))};document.getElementsByTagName("link");const c=document.querySelector("meta[property=csp-nonce]"),v=c?.nonce||c?.getAttribute("nonce");n=o(r.map(d=>{if(d=To(d),d in Ra)return;Ra[d]=!0;const h=d.endsWith(".css"),p=h?'[rel="stylesheet"]':"";if(document.querySelector(`link[href="${d}"]${p}`))return;const b=document.createElement("link");if(b.rel=h?"stylesheet":$o,h||(b.as="script"),b.crossOrigin="",b.href=d,v&&b.setAttribute("nonce",v),document.head.appendChild(b),h)return new Promise((y,x)=>{b.addEventListener("load",y),b.addEventListener("error",()=>x(new Error(`Unable to preload CSS for ${d}`)))})}))}function i(o){const c=new Event("vite:preloadError",{cancelable:!0});if(c.payload=o,window.dispatchEvent(c),!c.defaultPrevented)throw o}return n.then(o=>{for(const c of o||[])c.status==="rejected"&&i(c.reason);return e().catch(i)})};function Le(){return typeof window<"u"&&!!window.__TAURI_INTERNALS__}async function qe(t,e){if(!Le())throw new Error(`Tauri runtime not available for command "${t}"`);const{invoke:r}=await Po(async()=>{const{invoke:s}=await Promise.resolve().then(()=>Ao);return{invoke:s}},void 0);return r(t,e)}const Ae={loadInitialState:()=>qe("load_initial_state"),saveSession:t=>qe("save_session",{session:t}),saveFavorites:t=>qe("save_favorites_cmd",{favorites:t}),saveRecentTracks:t=>qe("save_recent_tracks_cmd",{tracks:t}),saveUserPlaylists:t=>qe("save_user_playlists_cmd",{playlists:t}),saveSearchHistory:t=>qe("save_search_history_cmd",{history:t}),getHomeFeed:(t=!1)=>qe("get_home_feed",{forceRefresh:t}),getGenres:(t=!1)=>qe("get_genres",{forceRefresh:t}),getGenrePlaylists:t=>qe("get_genre_playlists",{genreId:t}),searchAll:t=>qe("search_all",{query:t}),getSearchSuggestions:t=>qe("get_search_suggestions",{query:t}),getArtist:(t,e)=>qe("get_artist",{artistId:t,fallbackName:e}),getAlbumTracks:t=>qe("get_album_tracks",{albumId:t}),getPlaylistTracks:t=>qe("get_playlist_tracks",{playlistId:t}),getRadioTracks:t=>qe("get_radio_tracks",{trackId:t}),getLyrics:(t,e,r)=>qe("get_lyrics",{title:t,artist:e,durationSecs:r??null}),resolveAudioStream:t=>qe("resolve_audio_stream",{track:t}),prefetchAudio:t=>qe("prefetch_audio",{track:t}),cancelStaleStreams:t=>qe("cancel_stale_streams",{keepIds:t}),getCacheBytes:()=>qe("get_cache_bytes"),clearAudioCache:()=>qe("clear_audio_cache"),spinStatus:()=>qe("spin_status"),spinStartLocal:()=>qe("spin_start_local"),spinStopLocal:()=>qe("spin_stop_local"),spinSetAllowControls:t=>qe("spin_set_allow_controls",{allow:t}),spinStartRemoteHost:()=>qe("spin_start_remote_host"),spinStopRemote:()=>qe("spin_stop_remote"),spinJoinRemote:t=>qe("spin_join_remote",{code:t}),spinSyncSnapshot:t=>qe("spin_sync_snapshot",{isPlaying:t.isPlaying,positionSecs:t.positionSecs,sourceName:t.sourceName,currentIndex:t.currentIndex,queue:t.queue})};function ai(t,e=!1){return window.__TAURI_INTERNALS__.transformCallback(t,e)}async function ua(t,e={},r){return window.__TAURI_INTERNALS__.invoke(t,e,r)}const Ao=Object.freeze(Object.defineProperty({__proto__:null,invoke:ua,transformCallback:ai},Symbol.toStringTag,{value:"Module"}));var Ca;(function(t){t.WINDOW_RESIZED="tauri://resize",t.WINDOW_MOVED="tauri://move",t.WINDOW_CLOSE_REQUESTED="tauri://close-requested",t.WINDOW_DESTROYED="tauri://destroyed",t.WINDOW_FOCUS="tauri://focus",t.WINDOW_BLUR="tauri://blur",t.WINDOW_SCALE_FACTOR_CHANGED="tauri://scale-change",t.WINDOW_THEME_CHANGED="tauri://theme-changed",t.WINDOW_CREATED="tauri://window-created",t.WINDOW_SUSPENDED="tauri://suspended",t.WINDOW_RESUMED="tauri://resumed",t.WEBVIEW_CREATED="tauri://webview-created",t.DRAG_ENTER="tauri://drag-enter",t.DRAG_OVER="tauri://drag-over",t.DRAG_DROP="tauri://drag-drop",t.DRAG_LEAVE="tauri://drag-leave"})(Ca||(Ca={}));async function Eo(t,e){window.__TAURI_EVENT_PLUGIN_INTERNALS__.unregisterListener(t,e),await ua("plugin:event|unlisten",{event:t,eventId:e})}async function zs(t,e,r){var s;const n=(s=void 0)!==null&&s!==void 0?s:{kind:"Any"};return ua("plugin:event|listen",{event:t,target:n,handler:ai(e)}).then(i=>async()=>Eo(t,i))}class Io{decks;activeIndex=0;volume=1;crossfadeSecs=0;crossfadeFired=!1;fadeRaf=null;loading=!1;callbacks;constructor(e){this.callbacks=e,this.decks=[this.createDeck(0),this.createDeck(1)]}createDeck(e){const r=document.createElement("audio");return r.preload="auto",r.addEventListener("timeupdate",()=>{if(e!==this.activeIndex)return;const s=Number.isFinite(r.currentTime)?r.currentTime:0,n=Number.isFinite(r.duration)?r.duration:0;this.callbacks.onTimeUpdate(s,n),this.crossfadeSecs>0&&!this.crossfadeFired&&n>this.crossfadeSecs+2&&n-s<=this.crossfadeSecs&&(this.crossfadeFired=!0,this.callbacks.onCrossfadeTrigger())}),r.addEventListener("play",()=>{e===this.activeIndex&&this.callbacks.onPlayStateChange(!0)}),r.addEventListener("pause",()=>{e===this.activeIndex&&this.callbacks.onPlayStateChange(!1)}),r.addEventListener("ended",()=>{e===this.activeIndex&&this.callbacks.onEnded()}),r.addEventListener("error",()=>{if(e===this.activeIndex&&r.src&&!this.loading){const s=r.error?.code,n=r.error?.message||(s?`Audio error (code ${s})`:"Audio playback error");console.error("[GyroTube audio error]",{src:r.src,code:s,message:r.error?.message,networkState:r.networkState,readyState:r.readyState}),this.callbacks.onError(n)}}),r}cancelFade(){this.fadeRaf!==null&&(cancelAnimationFrame(this.fadeRaf),this.fadeRaf=null)}get activeDeck(){return this.decks[this.activeIndex]}setVolume(e){this.volume=Math.max(0,Math.min(1,e)),this.activeDeck.volume=this.volume}setCrossfadeSecs(e){this.crossfadeSecs=Math.max(0,Math.min(10,Math.round(e)))}async loadAndPlay(e,r=0,s=!1){this.cancelFade(),this.crossfadeFired=!1,this.loading=!0;const n=this.activeIndex,i=s?n===0?1:0:n,o=this.decks[n],c=this.decks[i];if(this.activeIndex=i,c.src=e,c.load(),r>0){const v=r;c.addEventListener("loadedmetadata",()=>{c.src===e&&Number.isFinite(v)&&(c.currentTime=v)},{once:!0})}try{if(s&&this.crossfadeSecs>0&&n!==i){const v=o.volume;c.volume=.01,await c.play();const d=this.crossfadeSecs*1e3,h=performance.now(),p=b=>{const y=Math.min(1,(b-h)/d);o.volume=Math.max(0,v*(1-y)),c.volume=Math.min(1,this.volume*y),y<1&&this.activeIndex===i?this.fadeRaf=requestAnimationFrame(p):(this.fadeRaf=null,this.activeIndex!==n&&(o.pause(),o.removeAttribute("src"),o.load()))};this.fadeRaf=requestAnimationFrame(p)}else n!==i&&(o.pause(),o.removeAttribute("src"),o.load()),c.volume=this.volume,await c.play()}catch(v){throw console.error("[GyroTube play() failed]",{url:e,err:v,canPlayWebm:c.canPlayType("audio/webm"),canPlayMp4:c.canPlayType("audio/mp4"),errorCode:c.error?.code,errorMessage:c.error?.message}),v}finally{this.loading=!1}}async togglePlay(){const e=this.activeDeck;e.src&&(e.paused?await e.play():e.pause())}seek(e){const r=this.activeDeck;!r.src||!Number.isFinite(e)||(r.currentTime=Math.max(0,e))}stop(){this.cancelFade();for(const e of this.decks)e.pause(),e.removeAttribute("src"),e.load();this.callbacks.onPlayStateChange(!1),this.callbacks.onTimeUpdate(0,0)}updateMediaSession(e,r){if(!("mediaSession"in navigator))return;if(!e){navigator.mediaSession.metadata=null;return}const s=e.cover_large_url||e.cover_url;navigator.mediaSession.metadata=new MediaMetadata({title:e.title,artist:e.artist,album:e.album??"",artwork:s?[{src:s,sizes:"512x512",type:"image/jpeg"}]:[]}),navigator.mediaSession.setActionHandler("play",r.onPlay),navigator.mediaSession.setActionHandler("pause",r.onPause),navigator.mediaSession.setActionHandler("previoustrack",r.onPrev),navigator.mediaSession.setActionHandler("nexttrack",r.onNext),navigator.mediaSession.setActionHandler("seekto",n=>{n.seekTime!=null&&r.onSeek(n.seekTime)})}}async function Oa(t){return new Promise(e=>{const r=new Image;r.crossOrigin="anonymous",r.referrerPolicy="no-referrer",r.onload=()=>{try{const s=document.createElement("canvas");s.width=24,s.height=24;const n=s.getContext("2d");if(!n)return e(null);n.drawImage(r,0,0,24,24);const i=n.getImageData(0,0,24,24).data;let o=0,c=0,v=0,d=0;for(let h=0;h<i.length;h+=4){const p=i[h],b=i[h+1],y=i[h+2],x=Math.max(p,b,y),A=Math.min(p,b,y),k=x===0?0:(x-A)/x,P=(x+A)/510;if(P<.08||P>.92)continue;const E=.25+k*2.5;o+=p*E,c+=b*E,v+=y*E,d+=E}if(d===0)return e(null);e([Math.round(o/d),Math.round(c/d),Math.round(v/d)])}catch{e(null)}},r.onerror=()=>e(null),r.src=t})}class Mo{#r=q("home");get activeTab(){return a(this.#r)}set activeTab(e){g(this.#r,e,!0)}#o=q(Qe({kind:"home"}));get activeView(){return a(this.#o)}set activeView(e){g(this.#o,e,!0)}#e=q(!0);get sidebarOpen(){return a(this.#e)}set sidebarOpen(e){g(this.#e,e,!0)}#i=q("player");get sidebarTab(){return a(this.#i)}set sidebarTab(e){g(this.#i,e,!0)}#s=q(!1);get spinOpen(){return a(this.#s)}set spinOpen(e){g(this.#s,e,!0)}#t=q(!1);get partyMode(){return a(this.#t)}set partyMode(e){g(this.#t,e,!0)}#n=q(!1);get settingsOpen(){return a(this.#n)}set settingsOpen(e){g(this.#n,e,!0)}#a=q(Qe([]));get queue(){return a(this.#a)}set queue(e){g(this.#a,e,!0)}#l=q(Qe([]));get unshuffledQueue(){return a(this.#l)}set unshuffledQueue(e){g(this.#l,e,!0)}#f=q(null);get currentIndex(){return a(this.#f)}set currentIndex(e){g(this.#f,e,!0)}#c=q("Not playing");get sourceName(){return a(this.#c)}set sourceName(e){g(this.#c,e,!0)}#h=q(!1);get isPlaying(){return a(this.#h)}set isPlaying(e){g(this.#h,e,!0)}#g=q(!1);get isLoadingAudio(){return a(this.#g)}set isLoadingAudio(e){g(this.#g,e,!0)}#u=q(0);get positionSecs(){return a(this.#u)}set positionSecs(e){g(this.#u,e,!0)}#p=q(0);get durationSecs(){return a(this.#p)}set durationSecs(e){g(this.#p,e,!0)}#m=q("YTM");get qualityLabel(){return a(this.#m)}set qualityLabel(e){g(this.#m,e,!0)}#k=q(1);get volume(){return a(this.#k)}set volume(e){g(this.#k,e,!0)}#b=q(!1);get shuffle(){return a(this.#b)}set shuffle(e){g(this.#b,e,!0)}#v=q(!1);get repeat(){return a(this.#v)}set repeat(e){g(this.#v,e,!0)}#_=q(0);get crossfadeSecs(){return a(this.#_)}set crossfadeSecs(e){g(this.#_,e,!0)}#y=q(!0);get autoplayRelated(){return a(this.#y)}set autoplayRelated(e){g(this.#y,e,!0)}#d=q(!0);get backgroundPlayback(){return a(this.#d)}set backgroundPlayback(e){g(this.#d,e,!0)}#x=q("system");get themeMode(){return a(this.#x)}set themeMode(e){g(this.#x,e,!0)}#w=q("Auto");get language(){return a(this.#w)}set language(e){g(this.#w,e,!0)}#S=q(null);get ambientRgb(){return a(this.#S)}set ambientRgb(e){g(this.#S,e,!0)}#$=q(Qe({type:"None"}));get lyrics(){return a(this.#$)}set lyrics(e){g(this.#$,e,!0)}#T=q(null);get activeLyricIndex(){return a(this.#T)}set activeLyricIndex(e){g(this.#T,e,!0)}#P=q(Qe([]));get suggestedTracks(){return a(this.#P)}set suggestedTracks(e){g(this.#P,e,!0)}#A=q(Qe([]));get favorites(){return a(this.#A)}set favorites(e){g(this.#A,e,!0)}#E=q(Qe([]));get recentTracks(){return a(this.#E)}set recentTracks(e){g(this.#E,e,!0)}#I=q(Qe([]));get userPlaylists(){return a(this.#I)}set userPlaylists(e){g(this.#I,e,!0)}#M=q(Qe([]));get searchHistory(){return a(this.#M)}set searchHistory(e){g(this.#M,e,!0)}#N=q(null);get homeFeed(){return a(this.#N)}set homeFeed(e){g(this.#N,e,!0)}#j=q(!1);get homeLoading(){return a(this.#j)}set homeLoading(e){g(this.#j,e,!0)}#q=q(Qe([]));get genres(){return a(this.#q)}set genres(e){g(this.#q,e,!0)}#z=q("");get searchQuery(){return a(this.#z)}set searchQuery(e){g(this.#z,e,!0)}#L=q(null);get searchResults(){return a(this.#L)}set searchResults(e){g(this.#L,e,!0)}#R=q(!1);get searchLoading(){return a(this.#R)}set searchLoading(e){g(this.#R,e,!0)}#C=q(Qe({local_active:!1,local_url:"",local_qr_svg:null,allow_controls:!1,guest_count:0,added_count:0,remote_role:"idle",remote_ticket:null,remote_qr_svg:null}));get spinStatus(){return a(this.#C)}set spinStatus(e){g(this.#C,e,!0)}#O=q(Qe([]));get toasts(){return a(this.#O)}set toasts(e){g(this.#O,e,!0)}toastSeq=0;loadGen=0;engine;#F=me(()=>this.currentIndex!=null&&this.currentIndex>=0&&this.currentIndex<this.queue.length?this.queue[this.currentIndex]:null);get currentTrack(){return a(this.#F)}set currentTrack(e){g(this.#F,e)}#D=me(()=>this.currentIndex!=null&&this.currentIndex+1<this.queue.length?this.queue[this.currentIndex+1]:null);get nextTrack(){return a(this.#D)}set nextTrack(e){g(this.#D,e)}constructor(){this.engine=new Io({onTimeUpdate:(e,r)=>{this.positionSecs=e,r>0&&(this.durationSecs=r),this.updateActiveLyric(e)},onPlayStateChange:e=>{this.isPlaying=e,e&&(this.isLoadingAudio=!1),this.syncSpinSnapshot()},onEnded:()=>{this.handleTrackEnded()},onCrossfadeTrigger:()=>{this.next(!0)},onError:e=>{this.isLoadingAudio=!1,this.showToast(e)}})}async bootstrap(){if(this.applyTheme(this.themeMode),!!Le()){try{const e=await Ae.loadInitialState();this.favorites=e.favorites,this.recentTracks=e.recent_tracks,this.userPlaylists=e.user_playlists,this.searchHistory=e.search_history;const r=e.session;this.sidebarOpen=!r.sidebar_collapsed,this.queue=r.queue,this.unshuffledQueue=r.unshuffled_queue,this.currentIndex=r.current_index,this.sourceName=r.source_name||"Not playing",this.volume=r.user_volume??1,this.shuffle=r.shuffle,this.repeat=r.repeat,this.crossfadeSecs=r.crossfade_secs??0,this.autoplayRelated=r.autoplay_related??!0,this.backgroundPlayback=r.background_playback??!0,this.language=r.language??"Auto",this.themeMode=r.theme_mode||"system",this.engine.setVolume(this.volume),this.engine.setCrossfadeSecs(this.crossfadeSecs),this.applyTheme(this.themeMode);const s=this.currentTrack;if(s){this.durationSecs=s.duration_secs??0;const n=s.cover_large_url||s.cover_url;n&&Oa(n).then(i=>{this.currentTrack?.id===s.id&&(this.ambientRgb=i)}),Ae.getLyrics(s.title,s.artist,s.duration_secs).then(i=>{this.currentTrack?.id===s.id&&(this.lyrics=i)})}}catch(e){this.showToast(String(e))}this.loadHome(),this.loadGenres(),this.refreshSpinStatus(),this.bindNativeEvents()}}async bindNativeEvents(){if(Le())try{await zs("tray:action",e=>{e.payload==="toggle_play"?this.togglePlay():e.payload==="next"?this.next():e.payload==="prev"&&this.prev()}),await zs("spin:command",e=>{const r=e.payload;r.kind==="toggle_play_pause"?this.togglePlay():r.kind==="next_track"?this.next():r.kind==="prev_track"?this.prev():r.kind==="move_track"?this.moveQueueItem(r.from,r.to):r.kind==="add_track"&&Ae.searchAll(r.video_id).then(s=>{const n=s.tracks.find(i=>i.id===r.video_id)??s.tracks[0];n&&(this.enqueueTrack(n),this.showToast(`${r.guest_name} added ${n.title}`))})}),await zs("spin:remote-sync",e=>{const{payload:r,error:s}=e.payload;if(s){this.showToast(s);return}if(!r?.track)return;const n=Math.max(0,Date.now()-r.sent_at_ms),i=r.position_secs+(r.is_playing?n/1e3:0);this.currentTrack?.id!==r.track.id?(this.queue=[r.track],this.unshuffledQueue=[r.track],this.sourceName=`SPIN · ${r.host_name||"Host"}`,this.playIndex(0,!1,i)):(Math.abs(this.positionSecs-i)>1.8&&this.seek(i),r.is_playing!==this.isPlaying&&this.engine.togglePlay())})}catch{}}applyTheme(e){if(this.themeMode=e,typeof document>"u")return;const r=document.documentElement;e==="system"?r.removeAttribute("data-theme"):r.setAttribute("data-theme",e)}setThemeMode(e){this.applyTheme(e),this.persistSession()}setCrossfadeSecs(e){this.crossfadeSecs=Math.max(0,Math.min(10,Math.round(e))),this.engine.setCrossfadeSecs(this.crossfadeSecs),this.persistSession()}setAutoplay(e){this.autoplayRelated=e,this.persistSession()}setBackgroundPlayback(e){this.backgroundPlayback=e,this.persistSession()}setVolume(e){this.volume=Math.max(0,Math.min(1,e)),this.engine.setVolume(this.volume),this.persistSession()}showToast(e){const r=++this.toastSeq;this.toasts=[...this.toasts,{id:r,message:e}],window.setTimeout(()=>{this.toasts=this.toasts.filter(s=>s.id!==r)},3200)}navigateTab(e){this.activeTab=e,this.activeView={kind:e}}openArtist(e,r){this.activeTab="explore",this.activeView={kind:"artist",artistId:e,artistName:r}}openAlbum(e,r,s,n=!1){this.activeView={kind:"album",id:e,title:r,coverUrl:s,isPlaylist:n}}async loadHome(e=!1){if(Le()){this.homeLoading=!0;try{this.homeFeed=await Ae.getHomeFeed(e)}catch(r){this.showToast(String(r))}finally{this.homeLoading=!1}}}async loadGenres(e=!1){if(Le())try{this.genres=await Ae.getGenres(e)}catch{}}async runSearch(e){const r=e.trim();if(r&&(this.searchQuery=r,this.activeTab="explore",this.activeView={kind:"explore"},this.searchHistory=[r,...this.searchHistory.filter(s=>s.toLowerCase()!==r.toLowerCase())].slice(0,8),Le())){Ae.saveSearchHistory(this.searchHistory),this.searchLoading=!0;try{this.searchResults=await Ae.searchAll(r)}catch(s){this.showToast(String(s))}finally{this.searchLoading=!1}}}clearSearchHistory(){this.searchHistory=[],Le()&&Ae.saveSearchHistory([])}isFavorite(e){return this.favorites.some(r=>r.id===e)}toggleFavorite(e){this.isFavorite(e.id)?this.favorites=this.favorites.filter(r=>r.id!==e.id):this.favorites=[e,...this.favorites],Le()&&Ae.saveFavorites(this.favorites)}replaceQueueAndPlay(e,r,s){e.length&&(this.queue=[...e],this.unshuffledQueue=[...e],this.sourceName=s,this.shuffle=!1,this.playIndex(Math.max(0,Math.min(r,e.length-1))))}enqueueTrack(e){this.queue=[...this.queue,e],this.showToast(`Added to queue: ${e.title}`),this.currentIndex==null?this.playIndex(0):this.persistSession()}async startRadioFromCurrent(){const e=this.currentTrack;if(!(!e||!Le()))try{const s=(await Ae.getRadioTracks(e.id)).filter(n=>n.id!==e.id);if(s.length===0){this.showToast("No related radio tracks found");return}this.queue=[e,...s],this.unshuffledQueue=[...this.queue],this.currentIndex=0,this.shuffle=!1,this.sourceName=`Radio · ${e.title}`,this.persistSession(),this.syncSpinSnapshot(),this.showToast(`Radio started · ${s.length} songs queued`)}catch(r){this.showToast(String(r))}}removeQueueIndex(e){if(e<0||e>=this.queue.length)return;const r=this.queue.filter((s,n)=>n!==e);if(this.queue=r,this.currentIndex!=null){if(r.length===0)this.currentIndex=null,this.engine.stop();else if(e===this.currentIndex){const s=Math.min(e,r.length-1);this.playIndex(s)}else e<this.currentIndex&&(this.currentIndex-=1);this.persistSession()}}moveQueueItem(e,r){if(e===r||e<0||r<0||e>=this.queue.length||r>=this.queue.length)return;const s=[...this.queue],[n]=s.splice(e,1);s.splice(r,0,n),this.queue=s,this.currentIndex===e?this.currentIndex=r:this.currentIndex!=null&&(e<this.currentIndex&&r>=this.currentIndex?this.currentIndex-=1:e>this.currentIndex&&r<=this.currentIndex&&(this.currentIndex+=1)),this.persistSession()}clearQueue(){this.queue=[],this.unshuffledQueue=[],this.currentIndex=null,this.engine.stop(),this.lyrics={type:"None"},this.ambientRgb=null,this.persistSession()}async playIndex(e,r=!1,s=0){if(e<0||e>=this.queue.length)return;const n=this.queue[e],i=++this.loadGen;this.currentIndex=e,this.isLoadingAudio=!0,this.positionSecs=0,this.durationSecs=n.duration_secs??0,this.lyrics={type:"None"},this.activeLyricIndex=null,this.recentTracks=[n,...this.recentTracks.filter(c=>c.id!==n.id)].slice(0,30),Le()&&Ae.saveRecentTracks(this.recentTracks);const o=n.cover_large_url||n.cover_url;if(o?Oa(o).then(c=>{this.loadGen===i&&(this.ambientRgb=c)}):this.ambientRgb=null,this.engine.updateMediaSession(n,{onPlay:()=>void this.togglePlay(),onPause:()=>void this.togglePlay(),onPrev:()=>void this.prev(),onNext:()=>void this.next(),onSeek:c=>this.seek(c)}),this.persistSession(),!Le()){this.isLoadingAudio=!1;return}try{Ae.cancelStaleStreams([n.id]);const c=await Ae.resolveAudioStream(n);if(this.loadGen!==i)return;this.qualityLabel=c.quality_label,await this.engine.loadAndPlay(c.url,s,r),this.syncSpinSnapshot(),Ae.getLyrics(n.title,n.artist,n.duration_secs).then(v=>{this.loadGen===i&&(this.lyrics=v)}),Ae.getRadioTracks(n.id).then(v=>{if(this.loadGen===i){const d=new Set(this.queue.map(h=>h.id));this.suggestedTracks=v.filter(h=>!d.has(h.id)).slice(0,12)}})}catch(c){this.loadGen===i&&(this.isLoadingAudio=!1,this.showToast(String(c)))}}async togglePlay(){if(!(!this.currentTrack||this.currentIndex==null)){if(!this.engine.activeDeck.src){await this.playIndex(this.currentIndex);return}await this.engine.togglePlay()}}seek(e){this.positionSecs=e,this.engine.seek(e),this.updateActiveLyric(e)}async prev(){if(this.positionSecs>3){this.seek(0);return}this.currentIndex!=null&&this.currentIndex>0&&await this.playIndex(this.currentIndex-1)}async next(e=!1){if(this.currentIndex!=null){if(this.currentIndex+1<this.queue.length)await this.playIndex(this.currentIndex+1,e);else if(this.repeat&&this.queue.length>0)await this.playIndex(0,e);else if(this.autoplayRelated&&this.suggestedTracks.length>0){const r=this.suggestedTracks[0];this.suggestedTracks=this.suggestedTracks.slice(1),this.queue=[...this.queue,r],await this.playIndex(this.queue.length-1,e)}}}async handleTrackEnded(){this.currentIndex!=null&&(this.currentIndex+1<this.queue.length||this.repeat||this.autoplayRelated)&&await this.next(!1)}toggleShuffle(){if(this.shuffle=!this.shuffle,this.queue.length<=1||this.currentIndex==null)return;const e=this.queue[this.currentIndex];if(this.shuffle){this.unshuffledQueue=[...this.queue];const r=this.queue.filter((s,n)=>n!==this.currentIndex);for(let s=r.length-1;s>0;s--){const n=Math.floor(Math.random()*(s+1));[r[s],r[n]]=[r[n],r[s]]}this.queue=[e,...r],this.currentIndex=0}else if(this.unshuffledQueue.length>0){const r=this.unshuffledQueue.findIndex(s=>s.id===e.id);this.queue=[...this.unshuffledQueue],this.currentIndex=r>=0?r:0}this.persistSession()}toggleRepeat(){this.repeat=!this.repeat,this.persistSession()}updateActiveLyric(e){if(this.lyrics.type!=="Synced")return;const r=this.lyrics.lines;let s=null;for(let n=r.length-1;n>=0;n--)if(r[n].time_secs<=e+.15){s=n;break}s!==this.activeLyricIndex&&(this.activeLyricIndex=s)}async refreshSpinStatus(){if(Le())try{this.spinStatus=await Ae.spinStatus(),this.syncSpinSnapshot()}catch{}}async syncSpinSnapshot(){if(Le()&&!(!this.spinStatus.local_active&&this.spinStatus.remote_role!=="host"))try{await Ae.spinSyncSnapshot({isPlaying:this.isPlaying,positionSecs:this.positionSecs,sourceName:this.sourceName,currentIndex:this.currentIndex,queue:this.queue})}catch{}}createUserPlaylist(e,r="",s="#2190a4",n=null){const i=e.trim()||"New playlist",o={id:`pl-${Date.now()}-${Math.random().toString(36).slice(2,7)}`,name:i,description:r.trim(),color_hex:s,icon_name:n,cover_path:null,tracks:[]};return this.userPlaylists=[o,...this.userPlaylists],Le()&&Ae.saveUserPlaylists(this.userPlaylists),o}updateUserPlaylist(e,r,s,n,i){const o=r.trim();o&&(this.userPlaylists=this.userPlaylists.map(c=>c.id===e?{...c,name:o,description:s.trim(),color_hex:n,icon_name:i}:c),Le()&&Ae.saveUserPlaylists(this.userPlaylists))}deleteUserPlaylist(e){this.userPlaylists=this.userPlaylists.filter(r=>r.id!==e),Le()&&Ae.saveUserPlaylists(this.userPlaylists)}addTrackToPlaylist(e,r){let s=!1;this.userPlaylists=this.userPlaylists.map(n=>n.id!==e||n.tracks.some(i=>i.id===r.id)?n:(s=!0,{...n,tracks:[...n.tracks,r]})),s&&(Le()&&Ae.saveUserPlaylists(this.userPlaylists),this.showToast(`Added "${r.title}" to playlist`))}removeTrackFromPlaylist(e,r){this.userPlaylists=this.userPlaylists.map(s=>s.id===e?{...s,tracks:s.tracks.filter(n=>n.id!==r)}:s),Le()&&Ae.saveUserPlaylists(this.userPlaylists)}async persistSession(){if(!Le())return;const e={sidebar_collapsed:!this.sidebarOpen,queue:this.queue,unshuffled_queue:this.unshuffledQueue,current_index:this.currentIndex,source_name:this.sourceName,user_volume:this.volume,shuffle:this.shuffle,repeat:this.repeat,position_secs:this.positionSecs,crossfade_secs:this.crossfadeSecs,language:this.language,background_playback:this.backgroundPlayback,autoplay_related:this.autoplayRelated,theme_mode:this.themeMode};try{await Ae.saveSession(e)}catch{}}}const l=new Mo;function dt(t){if(t==null||!Number.isFinite(t)||t<0)return"--:--";const e=Math.floor(t),r=Math.floor(e/60),s=e%60;return`${r}:${s.toString().padStart(2,"0")}`}function Ys(t){return t?t>=1e6?`${(t/1e6).toFixed(1)} M subscribers`:t>=1e3?`${(t/1e3).toFixed(1)} K subscribers`:`${t} subscribers`:""}var No=w('<img loading="lazy" referrerpolicy="no-referrer"/>'),jo=w('<div aria-hidden="true"> </div>');function Ge(t,e){ft(e,!0);let r=ut(e,"src",3,null),s=ut(e,"fallbackSrc",3,null),n=ut(e,"alt",3,""),i=ut(e,"class",3,""),o=ut(e,"fallbackIcon",3,"♪"),c=q(0),v=q(!1),d=q(null);Ut(()=>{g(d,r()||s()||null,!0),g(c,0),g(v,!1)});function h(){if(a(c)===0&&s()&&a(d)!==s()){g(c,1),g(d,s());return}if(a(c)<2&&a(d)){if(g(c,2),a(d).includes("googleusercontent.com")&&a(d).includes("=w")){const A=a(d).replace(/=w\d+-h\d+[^&?]*/,"=w226-h226-l90-rj");if(A!==a(d)){g(d,A,!0);return}}if(a(d).includes("i.ytimg.com")&&!a(d).includes("hqdefault")){const A=a(d).replace(/\/(maxresdefault|sddefault|mqdefault)\./,"/hqdefault.");if(A!==a(d)){g(d,A,!0);return}}}g(v,!0)}var p=Ne(),b=ke(p);{var y=A=>{var k=No();L(()=>{re(k,1,Hn(i()),"svelte-1gzalhk"),_t(k,"src",a(d)),_t(k,"alt",n())}),Ct("error",k,h),_(A,k)},x=A=>{var k=jo(),P=$(k,!0);L(()=>{re(k,1,`cover-fallback ${i()??""}`,"svelte-1gzalhk"),S(P,o())}),_(A,k)};z(b,A=>{a(d)&&!a(v)?A(y):A(x,-1)})}_(t,p),ht()}var qo=w('<p class="artist svelte-hawz4i"> </p>'),zo=w('<p class="muted">Loading tracks…</p>'),Lo=w('<div role="button" tabindex="0"><span> </span> <div class="track-meta"><span class="track-title"> </span> <span class="track-sub"> </span></div> <span class="muted numeric dur svelte-hawz4i"> </span> <button>♥</button> <button class="icon-btn">+</button></div>'),Ro=w('<div class="track-list"></div>'),Co=w('<div class="page-clamp"><div class="album-hero svelte-hawz4i"><button class="icon-btn back-btn svelte-hawz4i" title="Back">←</button> <div class="hero-row svelte-hawz4i"><!> <div class="album-meta svelte-hawz4i"><span class="type-pill svelte-hawz4i"> </span> <h1 class="svelte-hawz4i"> </h1> <!> <p class="muted small svelte-hawz4i"> </p></div></div></div> <div class="album-actions svelte-hawz4i"><button class="play-circle" title="Play all">▶</button> <button class="pill-btn">⇄ Shuffle</button></div> <!></div>');function Oo(t,e){ft(e,!0);let r=ut(e,"coverUrl",3,null),s=ut(e,"isPlaylist",3,!1),n=q(Qe([])),i=q(""),o=q(!1);Ut(()=>{const N=e.id,ae=s();if(g(i,e.title,!0),g(n,[],!0),!Le())return;g(o,!0),(ae?Ae.getPlaylistTracks(N):Ae.getAlbumTracks(N)).then(([B,J])=>{e.id===N&&(B&&g(i,B,!0),g(n,J,!0))}).catch(B=>l.showToast(String(B))).finally(()=>{e.id===N&&g(o,!1)})});let c=me(()=>Math.round(a(n).reduce((N,ae)=>N+(ae.duration_secs??0),0)/60));var v=Co(),d=f(v),h=f(d),p=u(h,2),b=f(p);{let N=me(()=>a(n)[0]?.cover_url);Ge(b,{class:"album-cover",get src(){return r()},get fallbackSrc(){return a(N)},get alt(){return a(i)}})}var y=u(b,2),x=f(y),A=$(x,!0),k=u(x,2),P=$(k,!0),E=u(k,2);{var W=N=>{var ae=qo(),T=$(ae,!0);L(()=>S(T,a(n)[0].artist)),_(N,ae)};z(E,N=>{a(n)[0]?.artist&&N(W)})}var K=u(E,2),he=$(K),I=u(d,2),M=f(I),Z=u(M,2),ue=u(I,2);{var j=N=>{var ae=zo();_(N,ae)},ie=N=>{var ae=Ro();Oe(ae,23,()=>a(n),(T,B)=>T.id+":"+B,(T,B,J)=>{const H=me(()=>l.currentTrack?.id===a(B).id);var G=Lo();let le;var oe=f(G);let ye;var F=$(oe,!0),ce=u(oe,2),D=f(ce),te=$(D,!0),se=u(D,2),pe=$(se,!0),ee=u(ce,2),Ie=$(ee,!0),we=u(ee,2);let Q;var fe=u(we,2);L((C,O)=>{le=re(G,1,"track-row",null,le,{active:a(H)}),ye=re(oe,1,"num numeric svelte-hawz4i",null,ye,{"active-num":a(H)}),S(F,a(J)+1),S(te,a(B).title),S(pe,a(B).artist),S(Ie,C),Q=re(we,1,"icon-btn",null,Q,{"fav-active":O})},[()=>dt(a(B).duration_secs),()=>l.isFavorite(a(B).id)]),m("click",G,()=>l.replaceQueueAndPlay(a(n),a(J),a(i))),m("keydown",G,C=>C.key==="Enter"&&l.replaceQueueAndPlay(a(n),a(J),a(i))),m("click",we,C=>{C.stopPropagation(),l.toggleFavorite(a(B))}),m("click",fe,C=>{C.stopPropagation(),l.enqueueTrack(a(B))}),_(T,G)}),_(N,ae)};z(ue,N=>{a(o)?N(j):a(n).length>0&&N(ie,1)})}L(()=>{S(A,s()?"PLAYLIST":"ALBUM"),S(P,a(i)),S(he,`${a(n).length??""} songs${a(c)?` • ${a(c)} min`:""}`),M.disabled=a(n).length===0,Z.disabled=a(n).length===0}),m("click",h,()=>l.navigateTab(l.activeTab)),m("click",M,()=>l.replaceQueueAndPlay(a(n),0,a(i))),m("click",Z,()=>{const N=[...a(n)].sort(()=>Math.random()-.5);l.replaceQueueAndPlay(N,0,a(i))}),_(t,v),ht()}kt(["click","keydown"]);var Fo=w('<img class="hero-bg svelte-1iungmq" alt=""/>'),Do=w('<p class="muted"> </p>'),Vo=w('<p class="muted">Loading artist…</p>'),Ho=w('<div role="button" tabindex="0"><!> <div class="track-meta"><span class="track-title"> </span> <span class="track-sub"> </span></div> <span class="muted numeric dur svelte-1iungmq"> </span> <button>♥</button> <button class="icon-btn">+</button></div>'),Qo=w('<button class="pill-btn toggle-btn svelte-1iungmq"> </button>'),Uo=w('<section class="section svelte-1iungmq"><h2 class="section-title">Popular songs</h2> <div class="track-list"></div> <!></section>'),Wo=w('<button class="media-card svelte-1iungmq"><!> <strong class="media-title svelte-1iungmq"> </strong> <span class="media-sub muted svelte-1iungmq"> </span></button>'),Bo=w('<section class="section svelte-1iungmq"><h2 class="section-title">Albums</h2> <div class="rail-scroll"></div></section>'),Go=w('<section class="section svelte-1iungmq"><div class="bio-card svelte-1iungmq"><h3 class="svelte-1iungmq">About the artist</h3> <p class="muted svelte-1iungmq"> </p></div></section>'),Yo=w("<!> <!> <!>",1),Ko=w('<div class="page-clamp"><div class="hero svelte-1iungmq"><!> <div class="hero-scrim svelte-1iungmq"><button class="icon-btn back-btn svelte-1iungmq" title="Back">←</button> <div class="hero-bottom svelte-1iungmq"><span class="verified svelte-1iungmq">✓ Artist</span> <h1 class="svelte-1iungmq"> </h1> <!> <div class="hero-actions svelte-1iungmq"><button class="pill-btn primary">▶ Play</button></div></div></div></div> <!></div>');function Jo(t,e){ft(e,!0);let r=q(null),s=q(!1),n=q(!1);Ut(()=>{const I=e.artistId,M=e.artistName;g(r,null),g(n,!1),Le()&&(g(s,!0),Ae.getArtist(I,M).then(Z=>{e.artistId===I&&g(r,Z,!0)}).catch(Z=>l.showToast(String(Z))).finally(()=>{e.artistId===I&&g(s,!1)}))});let i=me(()=>a(r)?a(n)?a(r).tracks:a(r).tracks.slice(0,5):[]);var o=Ko(),c=f(o),v=f(c);{var d=I=>{var M=Fo();L(()=>_t(M,"src",a(r).header_url)),_(I,M)};z(v,I=>{a(r)?.header_url&&I(d)})}var h=u(v,2),p=f(h),b=u(p,2),y=u(f(b),2),x=$(y,!0),A=u(y,2);{var k=I=>{var M=Do(),Z=$(M,!0);L(ue=>S(Z,ue),[()=>Ys(a(r).subscriber_count)]),_(I,M)};z(A,I=>{a(r)?.subscriber_count&&I(k)})}var P=u(A,2),E=$(P),W=u(c,2);{var K=I=>{var M=Vo();_(I,M)},he=I=>{var M=Yo(),Z=ke(M);{var ue=T=>{var B=Uo(),J=u(f(B),2);Oe(J,23,()=>a(i),le=>le.id,(le,oe,ye)=>{const F=me(()=>l.currentTrack?.id===a(oe).id);var ce=Ho();let D;var te=f(ce);Ge(te,{class:"track-cover",get src(){return a(oe).cover_url},alt:""});var se=u(te,2),pe=f(se),ee=$(pe,!0),Ie=u(pe,2),we=$(Ie),Q=u(se,2),fe=$(Q,!0),C=u(Q,2);let O;var R=u(C,2);L((de,Y)=>{D=re(ce,1,"track-row",null,D,{active:a(F)}),S(ee,a(oe).title),S(we,`${a(oe).artist??""}${a(oe).album?` • ${a(oe).album}`:""}`),S(fe,de),O=re(C,1,"icon-btn",null,O,{"fav-active":Y})},[()=>dt(a(oe).duration_secs),()=>l.isFavorite(a(oe).id)]),m("click",ce,()=>a(r)&&l.replaceQueueAndPlay(a(r).tracks,a(ye),a(r).name)),m("keydown",ce,de=>de.key==="Enter"&&a(r)&&l.replaceQueueAndPlay(a(r).tracks,a(ye),a(r).name)),m("click",C,de=>{de.stopPropagation(),l.toggleFavorite(a(oe))}),m("click",R,de=>{de.stopPropagation(),l.enqueueTrack(a(oe))}),_(le,ce)});var H=u(J,2);{var G=le=>{var oe=Qo(),ye=$(oe,!0);L(()=>S(ye,a(n)?"▴ Show less":`▾ Show ${a(r).tracks.length-5} more`)),m("click",oe,()=>g(n,!a(n))),_(le,oe)};z(H,le=>{a(r).tracks.length>5&&le(G)})}_(T,B)};z(Z,T=>{a(i).length>0&&T(ue)})}var j=u(Z,2);{var ie=T=>{var B=Bo(),J=u(f(B),2);Oe(J,21,()=>a(r).albums,H=>H.id,(H,G)=>{var le=Wo(),oe=f(le);Ge(oe,{class:"media-cover",get src(){return a(G).cover_url},get alt(){return a(G).title}});var ye=u(oe,2),F=$(ye,!0),ce=u(ye,2),D=$(ce,!0);L(()=>{S(F,a(G).title),S(D,a(G).year??a(G).artist)}),m("click",le,()=>l.openAlbum(a(G).id,a(G).title,a(G).cover_url)),_(H,le)}),_(T,B)};z(j,T=>{a(r).albums.length>0&&T(ie)})}var N=u(j,2);{var ae=T=>{var B=Go(),J=f(B),H=u(f(J),2),G=$(H,!0);L(()=>S(G,a(r).description)),_(T,B)};z(N,T=>{a(r).description&&T(ae)})}_(I,M)};z(W,I=>{a(s)?I(K):a(r)&&I(he,1)})}L(()=>{S(x,a(r)?.name??e.artistName),E.disabled=!a(r)?.tracks.length}),m("click",p,()=>l.navigateTab("explore")),m("click",E,()=>a(r)&&l.replaceQueueAndPlay(a(r).tracks,0,a(r).name)),_(t,o),ht()}kt(["click","keydown"]);var Xo=w('<p class="muted caption svelte-2o7gxz"> </p>'),Zo=w('<p class="muted empty-note svelte-2o7gxz">This playlist is empty. Add songs from your recent history or search results.</p>'),ec=w('<div role="button" tabindex="0"><!> <div class="track-meta"><span class="track-title"> </span> <span class="track-sub"> </span></div> <span class="muted numeric dur svelte-2o7gxz"> </span> <button title="Favorite">♥</button> <button class="icon-btn" title="Remove from playlist">✕</button></div>'),Fa=w('<div class="track-list"></div>'),tc=w('<div class="detail-head svelte-2o7gxz"><button class="icon-btn back-btn svelte-2o7gxz" title="Back">‹</button> <div class="detail-badge svelte-2o7gxz"> </div> <div class="detail-meta svelte-2o7gxz"><span class="kicker svelte-2o7gxz">LOCAL PLAYLIST</span> <h1 class="page-title svelte-2o7gxz"> </h1> <!> <p class="muted caption svelte-2o7gxz"> </p> <div class="detail-actions svelte-2o7gxz"><button class="pill-btn primary">▶ Play all</button> <button class="pill-btn">Edit</button> <button class="pill-btn danger svelte-2o7gxz">Delete</button></div></div></div> <!>',1),rc=w('<div class="playlist-card svelte-2o7gxz" role="button" tabindex="0"><span class="pl-badge svelte-2o7gxz"> </span> <div class="track-meta"><strong> </strong> <span class="muted small svelte-2o7gxz"> </span></div> <button class="icon-btn pl-play svelte-2o7gxz" title="Play playlist">▶</button></div>'),sc=w('<button class="pill-btn">▶ Play all</button>'),ac=w('<p class="muted">Songs you play will appear here.</p>'),nc=w('<button class="icon-btn" title="Add to playlist">☰</button>'),ic=w('<button class="pill-btn small-pill svelte-2o7gxz"> </button>'),lc=w('<div class="inline-picker svelte-2o7gxz"><span class="muted small svelte-2o7gxz">Add to:</span> <!></div>'),oc=w('<div role="button" tabindex="0"><!> <div class="track-meta"><span class="track-title"> </span> <span class="track-sub"> </span></div> <span class="muted numeric dur svelte-2o7gxz"> </span> <button title="Favorite">♥</button> <!> <button class="icon-btn" title="Add to queue">+</button></div> <!>',1),cc=w('<div class="page-head svelte-2o7gxz"><div><h1 class="page-title svelte-2o7gxz">Collection</h1> <p class="muted caption svelte-2o7gxz">Your local playlists, recent history, and favorite songs</p></div> <button class="pill-btn primary">+ New playlist</button></div> <section class="section svelte-2o7gxz"><div class="section-head svelte-2o7gxz"><h2 class="section-title">Local playlists</h2></div> <div class="playlists-grid svelte-2o7gxz"><div class="playlist-card fav-card svelte-2o7gxz" role="button" tabindex="0"><span class="pl-badge fav-badge svelte-2o7gxz">♥</span> <div class="track-meta"><strong>Favorites</strong> <span class="muted small svelte-2o7gxz"> </span></div> <button class="icon-btn pl-play svelte-2o7gxz" title="Play favorites">▶</button></div> <!></div></section> <section class="section svelte-2o7gxz"><div class="section-head svelte-2o7gxz"><h2 class="section-title">Recently played</h2> <!></div> <!></section>',1),uc=w('<button type="button"></button>'),vc=w('<button type="button"> </button>'),dc=w('<div class="page-clamp"><!></div> <dialog class="gt-dialog"><form class="modal-form svelte-2o7gxz"><div class="modal-head svelte-2o7gxz"><h3 class="svelte-2o7gxz"> </h3> <button type="button" class="icon-btn">✕</button></div> <label class="field svelte-2o7gxz"><span>Name</span> <input type="text" placeholder="My playlist" required="" class="svelte-2o7gxz"/></label> <label class="field svelte-2o7gxz"><span>Description</span> <input type="text" placeholder="Optional description" class="svelte-2o7gxz"/></label> <div class="field svelte-2o7gxz"><span>Color</span> <div class="swatch-row svelte-2o7gxz"></div></div> <div class="field svelte-2o7gxz"><span>Icon</span> <div class="icon-grid svelte-2o7gxz"></div></div> <div class="modal-actions svelte-2o7gxz"><button type="button" class="pill-btn">Cancel</button> <button type="submit" class="pill-btn primary"> </button></div></form></dialog>',1);function fc(t,e){ft(e,!0);const r=["#2190a4","#e01b24","#3584e4","#9141ac","#2ec27e","#e66100","#c061cb","#f5c211"],s=["♪","★","♥","⚡","🌙","☀","🔥","🎧","🎸","🎹","🚗","🏋"];let n=q(null),i=me(()=>a(n)?l.userPlaylists.find(D=>D.id===a(n))??null:null),o=q(null),c=q(null),v=q(""),d=q(""),h=q(Qe(r[0])),p=q("♪"),b=q(null);function y(){g(c,null),g(v,""),g(d,""),g(h,r[0],!0),g(p,"♪"),a(o)?.showModal()}function x(D){g(c,D.id,!0),g(v,D.name,!0),g(d,D.description||"",!0),g(h,D.color_hex||r[0],!0),g(p,D.icon_name||"♪",!0),a(o)?.showModal()}function A(D){if(D.preventDefault(),!!a(v).trim()){if(a(c))l.updateUserPlaylist(a(c),a(v),a(d),a(h),a(p));else{const te=l.createUserPlaylist(a(v),a(d),a(h),a(p));g(n,te.id,!0)}a(o)?.close()}}function k(D){l.deleteUserPlaylist(D.id),a(n)===D.id&&g(n,null)}var P=dc(),E=ke(P),W=f(E);{var K=D=>{var te=tc(),se=ke(te),pe=f(se),ee=u(pe,2),Ie=$(ee,!0),we=u(ee,2),Q=u(f(we),2),fe=$(Q,!0),C=u(Q,2);{var O=V=>{var Te=Xo(),ge=$(Te,!0);L(()=>S(ge,a(i).description)),_(V,Te)};z(C,V=>{a(i).description&&V(O)})}var R=u(C,2),de=$(R),Y=u(R,2),U=f(Y),ne=u(U,2),ve=u(ne,2),xe=u(se,2);{var _e=V=>{var Te=Zo();_(V,Te)},be=V=>{var Te=Fa();Oe(Te,23,()=>a(i).tracks,ge=>ge.id,(ge,Me,je)=>{const We=me(()=>l.currentTrack?.id===a(Me).id);var Pe=ec();let Xe;var at=f(Pe);Ge(at,{class:"track-cover",get src(){return a(Me).cover_url},alt:""});var lt=u(at,2),rt=f(lt),zt=$(rt,!0),wt=u(rt,2),Ue=$(wt),Be=u(lt,2),nt=$(Be,!0),ot=u(Be,2);let ar;var nr=u(ot,2);L((St,Er)=>{Xe=re(Pe,1,"track-row",null,Xe,{active:a(We)}),S(zt,a(Me).title),S(Ue,`${a(Me).artist??""}${a(Me).album?` • ${a(Me).album}`:""}`),S(nt,St),ar=re(ot,1,"icon-btn svelte-2o7gxz",null,ar,{"fav-active":Er})},[()=>dt(a(Me).duration_secs),()=>l.isFavorite(a(Me).id)]),m("click",Pe,()=>a(i)&&l.replaceQueueAndPlay(a(i).tracks,a(je),a(i).name)),m("keydown",Pe,St=>St.key==="Enter"&&a(i)&&l.replaceQueueAndPlay(a(i).tracks,a(je),a(i).name)),m("click",ot,St=>{St.stopPropagation(),l.toggleFavorite(a(Me))}),m("click",nr,St=>{St.stopPropagation(),a(i)&&l.removeTrackFromPlaylist(a(i).id,a(Me).id)}),_(ge,Pe)}),_(V,Te)};z(xe,V=>{a(i).tracks.length===0?V(_e):V(be,-1)})}L(()=>{bt(ee,`background: ${(a(i).color_hex||"var(--accent)")??""};`),S(Ie,a(i).icon_name||"♪"),S(fe,a(i).name),S(de,`${a(i).tracks.length??""} songs`),U.disabled=a(i).tracks.length===0}),m("click",pe,()=>g(n,null)),m("click",U,()=>a(i)&&l.replaceQueueAndPlay(a(i).tracks,0,a(i).name)),m("click",ne,()=>a(i)&&x(a(i))),m("click",ve,()=>a(i)&&k(a(i))),_(D,te)},he=D=>{var te=cc(),se=ke(te),pe=u(f(se),2),ee=u(se,2),Ie=u(f(ee),2),we=f(Ie),Q=u(f(we),2),fe=u(f(Q),2),C=$(fe),O=u(Q,2),R=u(we,2);Oe(R,17,()=>l.userPlaylists,be=>be.id,(be,V)=>{var Te=rc(),ge=f(Te),Me=$(ge,!0),je=u(ge,2),We=f(je),Pe=$(We,!0),Xe=u(We,2),at=$(Xe),lt=u(je,2);L(()=>{bt(ge,`background: ${(a(V).color_hex||"var(--accent)")??""};`),S(Me,a(V).icon_name||"♪"),S(Pe,a(V).name),S(at,`${a(V).tracks.length??""} songs`),lt.disabled=a(V).tracks.length===0}),m("click",Te,()=>g(n,a(V).id,!0)),m("keydown",Te,rt=>rt.key==="Enter"&&g(n,a(V).id,!0)),m("click",lt,rt=>{rt.stopPropagation(),a(V).tracks.length&&l.replaceQueueAndPlay(a(V).tracks,0,a(V).name)}),_(be,Te)});var de=u(ee,2),Y=f(de),U=u(f(Y),2);{var ne=be=>{var V=sc();m("click",V,()=>l.replaceQueueAndPlay(l.recentTracks,0,"Recent")),_(be,V)};z(U,be=>{l.recentTracks.length>0&&be(ne)})}var ve=u(Y,2);{var xe=be=>{var V=ac();_(be,V)},_e=be=>{var V=Fa();Oe(V,23,()=>l.recentTracks,Te=>Te.id,(Te,ge,Me)=>{const je=me(()=>l.currentTrack?.id===a(ge).id);var We=oc(),Pe=ke(We);let Xe;var at=f(Pe);Ge(at,{class:"track-cover",get src(){return a(ge).cover_url},alt:""});var lt=u(at,2),rt=f(lt),zt=$(rt,!0),wt=u(rt,2),Ue=$(wt),Be=u(lt,2),nt=$(Be,!0),ot=u(Be,2);let ar;var nr=u(ot,2);{var St=Ke=>{var Lt=nc();m("click",Lt,Ir=>{Ir.stopPropagation(),g(b,a(b)?.id===a(ge).id?null:a(ge),!0)}),_(Ke,Lt)};z(nr,Ke=>{l.userPlaylists.length>0&&Ke(St)})}var Er=u(nr,2),ys=u(Pe,2);{var xs=Ke=>{var Lt=lc(),Ir=u(f(Lt),2);Oe(Ir,17,()=>l.userPlaylists,Mr=>Mr.id,(Mr,Gt)=>{var Nr=ic(),_r=$(Nr);L(()=>S(_r,`${(a(Gt).icon_name||"♪")??""} ${a(Gt).name??""}`)),m("click",Nr,()=>{l.addTrackToPlaylist(a(Gt).id,a(ge)),g(b,null)}),_(Mr,Nr)}),_(Ke,Lt)};z(ys,Ke=>{a(b)?.id===a(ge).id&&Ke(xs)})}L((Ke,Lt)=>{Xe=re(Pe,1,"track-row",null,Xe,{active:a(je)}),S(zt,a(ge).title),S(Ue,`${a(ge).artist??""}${a(ge).album?` • ${a(ge).album}`:""}`),S(nt,Ke),ar=re(ot,1,"icon-btn svelte-2o7gxz",null,ar,{"fav-active":Lt})},[()=>dt(a(ge).duration_secs),()=>l.isFavorite(a(ge).id)]),m("click",Pe,()=>l.replaceQueueAndPlay(l.recentTracks,a(Me),"Recent")),m("keydown",Pe,Ke=>Ke.key==="Enter"&&l.replaceQueueAndPlay(l.recentTracks,a(Me),"Recent")),m("click",ot,Ke=>{Ke.stopPropagation(),l.toggleFavorite(a(ge))}),m("click",Er,Ke=>{Ke.stopPropagation(),l.enqueueTrack(a(ge))}),_(Te,We)}),_(be,V)};z(ve,be=>{l.recentTracks.length===0?be(xe):be(_e,-1)})}L(()=>{S(C,`${l.favorites.length??""} songs`),O.disabled=l.favorites.length===0}),m("click",pe,y),m("click",we,()=>l.favorites.length&&l.replaceQueueAndPlay(l.favorites,0,"Favorites")),m("keydown",we,be=>be.key==="Enter"&&l.favorites.length&&l.replaceQueueAndPlay(l.favorites,0,"Favorites")),_(D,te)};z(W,D=>{a(i)?D(K):D(he,-1)})}var I=u(E,2),M=f(I),Z=f(M),ue=f(Z),j=$(ue,!0),ie=u(ue,2),N=u(Z,2),ae=u(f(N),2),T=u(N,2),B=u(f(T),2),J=u(T,2),H=u(f(J),2);Oe(H,21,()=>r,pr,(D,te)=>{var se=uc();let pe;L(()=>{pe=re(se,1,"swatch svelte-2o7gxz",null,pe,{selected:a(h)===a(te)}),bt(se,`background: ${a(te)??""};`),_t(se,"aria-label",a(te))}),m("click",se,()=>g(h,a(te),!0)),_(D,se)});var G=u(J,2),le=u(f(G),2);Oe(le,21,()=>s,pr,(D,te)=>{var se=vc();let pe;var ee=$(se,!0);L(()=>{pe=re(se,1,"icon-choice svelte-2o7gxz",null,pe,{selected:a(p)===a(te)}),S(ee,a(te))}),m("click",se,()=>g(p,a(te),!0)),_(D,se)});var oe=u(G,2),ye=f(oe),F=u(ye,2),ce=$(F,!0);ms(I,D=>g(o,D),()=>a(o)),L(()=>{S(j,a(c)?"Edit playlist":"New playlist"),S(ce,a(c)?"Save":"Create")}),Ct("submit",M,A),m("click",ie,()=>a(o)?.close()),vs(ae,()=>a(v),D=>g(v,D)),vs(B,()=>a(d),D=>g(d,D)),m("click",ye,()=>a(o)?.close()),_(t,P),ht()}kt(["click","keydown"]);var hc=w('<span class="muted"> </span>'),pc=w('<section class="overview-col"><h3 class="col-title svelte-8hoooi">Top result</h3> <div class="top-result-card svelte-8hoooi" role="button" tabindex="0"><!> <h2 class="top-name svelte-8hoooi"> </h2> <div class="top-bottom svelte-8hoooi"><div class="top-meta svelte-8hoooi"><span class="type-pill svelte-8hoooi">Artist</span> <!></div> <button class="play-circle small-play svelte-8hoooi" title="Play">▶</button></div></div></section>'),_c=w('<button class="see-all svelte-8hoooi">See all →</button>'),Da=w('<div role="button" tabindex="0"><!> <div class="track-meta"><span class="track-title"> </span> <span class="track-sub"> </span></div> <span class="muted numeric dur svelte-8hoooi"> </span> <button title="Favorite">♥</button> <button class="icon-btn" title="Add to queue">+</button></div>'),gc=w('<section class="overview-col songs-col"><div class="col-head svelte-8hoooi"><h3 class="col-title svelte-8hoooi">Songs</h3> <!></div> <div class="track-list"></div></section>'),bc=w('<div class="overview-grid svelte-8hoooi"><!> <!></div>'),mc=w('<span class="muted small svelte-8hoooi"> </span>'),yc=w('<button class="artist-card svelte-8hoooi"><!> <strong> </strong> <!></button>'),xc=w('<section class="section svelte-8hoooi"><h2 class="section-title">Artists</h2> <div class="rail-scroll"></div></section>'),Va=w('<button class="media-card svelte-8hoooi"><!> <strong class="media-title svelte-8hoooi"> </strong> <span class="media-sub muted svelte-8hoooi"> </span></button>'),kc=w('<section class="section svelte-8hoooi"><h2 class="section-title">Albums</h2> <div class="rail-scroll"></div></section>'),wc=w('<section class="section svelte-8hoooi"><h2 class="section-title">Playlists</h2> <div class="rail-scroll"></div></section>'),Sc=w('<section class="section svelte-8hoooi"><h2 class="section-title">Songs</h2> <div class="track-list"></div></section>'),$c=w('<div class="filters svelte-8hoooi"><button>All</button> <button>Songs<!></button> <button>Albums<!></button> <button>Artists<!></button> <button>Playlists<!></button></div> <!> <!> <!> <!> <!>',1),Tc=w('<button class="mood-tile svelte-8hoooi"><span class="mood-dot svelte-8hoooi">♪</span> <span class="genre-name svelte-8hoooi"> </span> <span class="muted">›</span></button>'),Pc=w('<section class="section svelte-8hoooi"><h2 class="section-title">Moods &amp; moments</h2> <div class="genre-grid svelte-8hoooi"></div></section>'),Ac=w('<button class="genre-chip svelte-8hoooi"><span class="genre-bar svelte-8hoooi"></span> <span class="genre-name svelte-8hoooi"> </span> <span class="muted">›</span></button>'),Ec=w('<section class="section svelte-8hoooi"><h2 class="section-title">Music genres</h2> <div class="genre-grid svelte-8hoooi"></div></section>'),Ic=w('<h1 class="explore-title svelte-8hoooi">Explore</h1> <p class="muted caption svelte-8hoooi">Moods, moments, and music genres from YouTube Music</p> <!> <!>',1),Mc=w('<div class="page-clamp"><!></div>');function Nc(t,e){ft(e,!0);let r=q("all");const s=["#2190a4","#e66100","#3584e4","#9141ac","#2ec27e","#e01b24","#c061cb","#f5c211","#00b4d8","#625df5"],n=["chill","christmas","commute","energize","feel good","focus","gaming","party","romance","sad","sleep","workout"];function i(k){let P=2166136261;for(let E=0;E<k.length;E++)P^=k.toLowerCase().charCodeAt(E),P=Math.imul(P,16777619);return s[Math.abs(P)%s.length]}let o=me(()=>l.genres.filter(k=>n.some(P=>k.name.toLowerCase().includes(P)))),c=me(()=>l.genres.filter(k=>!n.some(P=>k.name.toLowerCase().includes(P)))),v=me(()=>l.searchResults),d=me(()=>a(v)?.artists[0]??null),h=me(()=>a(v)?.tracks.slice(0,4)??[]),p=me(()=>a(r)==="all"&&!!a(d)&&(a(v)?.artists.length??0)===1);var b=Mc(),y=f(b);{var x=k=>{var P=$c(),E=ke(P),W=f(E);let K;var he=u(W,2);let I;var M=u(f(he));{var Z=Q=>{var fe=vr();L(()=>S(fe,` · ${a(v).tracks.length??""}`)),_(Q,fe)};z(M,Q=>{a(v).tracks.length&&Q(Z)})}var ue=u(he,2);let j;var ie=u(f(ue));{var N=Q=>{var fe=vr();L(()=>S(fe,` · ${a(v).albums.length??""}`)),_(Q,fe)};z(ie,Q=>{a(v).albums.length&&Q(N)})}var ae=u(ue,2);let T;var B=u(f(ae));{var J=Q=>{var fe=vr();L(()=>S(fe,` · ${a(v).artists.length??""}`)),_(Q,fe)};z(B,Q=>{a(v).artists.length&&Q(J)})}var H=u(ae,2);let G;var le=u(f(H));{var oe=Q=>{var fe=vr();L(()=>S(fe,` · ${a(v).playlists.length??""}`)),_(Q,fe)};z(le,Q=>{a(v).playlists.length&&Q(oe)})}var ye=u(E,2);{var F=Q=>{var fe=bc(),C=f(fe);{var O=Y=>{var U=pc(),ne=u(f(U),2),ve=f(ne);Ge(ve,{class:"top-avatar",get src(){return a(d).avatar_url},get alt(){return a(d).name},fallbackIcon:"👤"});var xe=u(ve,2),_e=$(xe,!0),be=u(xe,2),V=f(be),Te=u(f(V),2);{var ge=je=>{var We=hc(),Pe=$(We,!0);L(Xe=>S(Pe,Xe),[()=>Ys(a(d).subscriber_count)]),_(je,We)};z(Te,je=>{a(d).subscriber_count&&je(ge)})}var Me=u(V,2);L(()=>S(_e,a(d).name)),m("click",ne,()=>l.openArtist(a(d).id,a(d).name)),m("keydown",ne,je=>je.key==="Enter"&&l.openArtist(a(d).id,a(d).name)),m("click",Me,je=>{je.stopPropagation(),a(h)[0]&&l.replaceQueueAndPlay([a(h)[0]],0,`Search: ${l.searchQuery}`)}),_(Y,U)};z(C,Y=>{a(d)&&Y(O)})}var R=u(C,2);{var de=Y=>{var U=gc(),ne=f(U),ve=u(f(ne),2);{var xe=be=>{var V=_c();m("click",V,()=>g(r,"songs")),_(be,V)};z(ve,be=>{a(v).tracks.length>4&&be(xe)})}var _e=u(ne,2);Oe(_e,21,()=>a(h),be=>be.id,(be,V)=>{const Te=me(()=>l.currentTrack?.id===a(V).id);var ge=Da();let Me;var je=f(ge);Ge(je,{class:"track-cover",get src(){return a(V).cover_url},alt:""});var We=u(je,2),Pe=f(We),Xe=$(Pe,!0),at=u(Pe,2),lt=$(at),rt=u(We,2),zt=$(rt,!0),wt=u(rt,2);let Ue;var Be=u(wt,2);L((nt,ot)=>{Me=re(ge,1,"track-row",null,Me,{active:a(Te)}),S(Xe,a(V).title),S(lt,`${a(V).artist??""}${a(V).album?` • ${a(V).album}`:""}`),S(zt,nt),Ue=re(wt,1,"icon-btn",null,Ue,{"fav-active":ot})},[()=>dt(a(V).duration_secs),()=>l.isFavorite(a(V).id)]),m("click",ge,()=>l.replaceQueueAndPlay([a(V)],0,`Search: ${l.searchQuery}`)),m("keydown",ge,nt=>nt.key==="Enter"&&l.replaceQueueAndPlay([a(V)],0,`Search: ${l.searchQuery}`)),m("click",wt,nt=>{nt.stopPropagation(),l.toggleFavorite(a(V))}),m("click",Be,nt=>{nt.stopPropagation(),l.enqueueTrack(a(V))}),_(be,ge)}),_(Y,U)};z(R,Y=>{a(h).length>0&&Y(de)})}_(Q,fe)};z(ye,Q=>{a(r)==="all"&&(a(d)||a(h).length>0)&&Q(F)})}var ce=u(ye,2);{var D=Q=>{var fe=xc(),C=u(f(fe),2);Oe(C,21,()=>a(v).artists,O=>O.id,(O,R)=>{var de=yc(),Y=f(de);Ge(Y,{class:"artist-avatar",get src(){return a(R).avatar_url},get alt(){return a(R).name},fallbackIcon:"👤"});var U=u(Y,2),ne=$(U,!0),ve=u(U,2);{var xe=_e=>{var be=mc(),V=$(be,!0);L(Te=>S(V,Te),[()=>Ys(a(R).subscriber_count)]),_(_e,be)};z(ve,_e=>{a(R).subscriber_count&&_e(xe)})}L(()=>S(ne,a(R).name)),m("click",de,()=>l.openArtist(a(R).id,a(R).name)),_(O,de)}),_(Q,fe)};z(ce,Q=>{(a(r)==="all"||a(r)==="artists")&&a(v).artists.length>0&&!a(p)&&Q(D)})}var te=u(ce,2);{var se=Q=>{var fe=kc(),C=u(f(fe),2);Oe(C,21,()=>a(v).albums,O=>O.id,(O,R)=>{var de=Va(),Y=f(de);Ge(Y,{class:"media-cover",get src(){return a(R).cover_url},get alt(){return a(R).title}});var U=u(Y,2),ne=$(U,!0),ve=u(U,2),xe=$(ve,!0);L(()=>{S(ne,a(R).title),S(xe,a(R).artist)}),m("click",de,()=>l.openAlbum(a(R).id,a(R).title,a(R).cover_url)),_(O,de)}),_(Q,fe)};z(te,Q=>{(a(r)==="all"||a(r)==="albums")&&a(v).albums.length>0&&Q(se)})}var pe=u(te,2);{var ee=Q=>{var fe=wc(),C=u(f(fe),2);Oe(C,21,()=>a(v).playlists,O=>O.id,(O,R)=>{var de=Va(),Y=f(de);Ge(Y,{class:"media-cover",get src(){return a(R).cover_url},get alt(){return a(R).name}});var U=u(Y,2),ne=$(U,!0),ve=u(U,2),xe=$(ve,!0);L(()=>{S(ne,a(R).name),S(xe,a(R).subtitle)}),m("click",de,()=>l.openAlbum(a(R).id,a(R).name,a(R).cover_url,!0)),_(O,de)}),_(Q,fe)};z(pe,Q=>{(a(r)==="all"||a(r)==="playlists")&&a(v).playlists.length>0&&Q(ee)})}var Ie=u(pe,2);{var we=Q=>{var fe=Sc(),C=u(f(fe),2);Oe(C,21,()=>a(v).tracks,O=>O.id,(O,R)=>{const de=me(()=>l.currentTrack?.id===a(R).id);var Y=Da();let U;var ne=f(Y);Ge(ne,{class:"track-cover",get src(){return a(R).cover_url},alt:""});var ve=u(ne,2),xe=f(ve),_e=$(xe,!0),be=u(xe,2),V=$(be),Te=u(ve,2),ge=$(Te,!0),Me=u(Te,2);let je;var We=u(Me,2);L((Pe,Xe)=>{U=re(Y,1,"track-row",null,U,{active:a(de)}),S(_e,a(R).title),S(V,`${a(R).artist??""}${a(R).album?` • ${a(R).album}`:""}`),S(ge,Pe),je=re(Me,1,"icon-btn",null,je,{"fav-active":Xe})},[()=>dt(a(R).duration_secs),()=>l.isFavorite(a(R).id)]),m("click",Y,()=>l.replaceQueueAndPlay([a(R)],0,`Search: ${l.searchQuery}`)),m("keydown",Y,Pe=>Pe.key==="Enter"&&l.replaceQueueAndPlay([a(R)],0,`Search: ${l.searchQuery}`)),m("click",Me,Pe=>{Pe.stopPropagation(),l.toggleFavorite(a(R))}),m("click",We,Pe=>{Pe.stopPropagation(),l.enqueueTrack(a(R))}),_(O,Y)}),_(Q,fe)};z(Ie,Q=>{a(r)==="songs"&&a(v).tracks.length>0&&Q(we)})}L(()=>{K=re(W,1,"svelte-8hoooi",null,K,{active:a(r)==="all"}),I=re(he,1,"svelte-8hoooi",null,I,{active:a(r)==="songs"}),j=re(ue,1,"svelte-8hoooi",null,j,{active:a(r)==="albums"}),T=re(ae,1,"svelte-8hoooi",null,T,{active:a(r)==="artists"}),G=re(H,1,"svelte-8hoooi",null,G,{active:a(r)==="playlists"})}),m("click",W,()=>g(r,"all")),m("click",he,()=>g(r,"songs")),m("click",ue,()=>g(r,"albums")),m("click",ae,()=>g(r,"artists")),m("click",H,()=>g(r,"playlists")),_(k,P)},A=k=>{var P=Ic(),E=u(ke(P),4);{var W=I=>{var M=Pc(),Z=u(f(M),2);Oe(Z,21,()=>a(o),ue=>ue.id,(ue,j)=>{const ie=me(()=>i(a(j).name));var N=Tc(),ae=f(N),T=u(ae,2),B=$(T,!0);L(()=>{bt(N,`background: linear-gradient(135deg, ${a(ie)??""}33, var(--surface) 68%); border-color: ${a(ie)??""}44;`),bt(ae,`background: ${a(ie)??""}33; color: ${a(ie)??""};`),S(B,a(j).name)}),m("click",N,()=>void l.runSearch(a(j).name)),_(ue,N)}),_(I,M)};z(E,I=>{a(o).length>0&&I(W)})}var K=u(E,2);{var he=I=>{var M=Ec(),Z=u(f(M),2);Oe(Z,21,()=>a(c),ue=>ue.id,(ue,j)=>{const ie=me(()=>i(a(j).name));var N=Ac(),ae=f(N),T=u(ae,2),B=$(T,!0);L(()=>{bt(N,`background: linear-gradient(90deg, ${a(ie)??""}26, var(--surface) 62%);`),bt(ae,`background: ${a(ie)??""};`),S(B,a(j).name)}),m("click",N,()=>void l.runSearch(a(j).name)),_(ue,N)}),_(I,M)};z(K,I=>{a(c).length>0&&I(he)})}_(k,P)};z(y,k=>{a(v)?k(x):k(A,-1)})}_(t,b),ht()}kt(["click","keydown"]);var jc=w('<button class="text-[var(--accent-bright)] hover:underline normal-case font-semibold">Clear</button>'),qc=w('<span class="text-[var(--text-muted)]"> </span><strong> </strong>',1),zc=w('<button><!> <span class="truncate"><!></span></button>'),Lc=w('<div class="absolute left-0 right-0 top-[calc(100%+8px)] rounded-2xl bg-[var(--bg-elevated)] border border-[var(--border-strong)] shadow-2xl overflow-hidden py-1.5 z-50" role="listbox"><div class="flex items-center justify-between px-3.5 py-1.5 text-[11px] font-bold uppercase tracking-wider text-[var(--text-muted)]"><span> </span> <!></div> <!></div>'),Rc=w('<header class="h-14 shrink-0 z-30 grid grid-cols-[auto_minmax(0,1fr)_auto] items-center gap-4 px-4 bg-[var(--bg-elevated)] border-b border-[var(--border)]"><div class="flex items-center gap-2.5 shrink-0"><button class="icon-btn" title="Toggle player sidebar"><!></button> <span class="w-4 h-4 rounded-full bg-gradient-to-br from-[#2eb0c7] to-[#2190a4] shadow-[0_0_12px_rgba(33,144,164,0.55)] shrink-0" aria-hidden="true"></span> <span class="font-extrabold text-[15px] tracking-tight hidden sm:inline">GyroTube</span> <span class="hidden md:inline-flex items-center px-2 py-0.5 rounded-full text-[11px] font-bold bg-[var(--accent-soft)] text-[var(--accent-bright)]">Music</span></div> <div class="w-full max-w-[460px] mx-auto relative"><div class="flex items-center gap-2.5 h-9 px-3.5 rounded-full bg-[var(--surface)] border border-[var(--border)] focus-within:border-[var(--accent-border)] focus-within:bg-[var(--surface-hover)] transition-colors"><!> <input type="search" class="w-full bg-transparent outline-none text-[13.5px] placeholder:text-[var(--text-muted)]" placeholder="Search songs, albums, artists…"/></div> <!></div> <div class="flex items-center gap-1.5 shrink-0"><button title="SPIN Mode"><!></button> <button title="Party Mode (F11)"><!></button> <span class="w-px h-5 bg-[var(--border-strong)] mx-0.5" aria-hidden="true"></span> <button title="Settings"><!></button></div></header>');function Cc(t,e){ft(e,!0);let r=q(""),s=q(Qe([])),n=q(!1),i=q(-1),o,c=me(()=>a(r).trim().length===0),v=me(()=>a(c)?l.searchHistory:a(s));function d(){g(i,-1),window.clearTimeout(o);const H=a(r).trim();if(!H){g(s,[],!0),g(n,l.searchHistory.length>0);return}g(n,!0),o=window.setTimeout(async()=>{if(Le())try{g(s,await Ae.getSearchSuggestions(H),!0),g(n,a(s).length>0)}catch{g(s,[],!0)}},180)}function h(H){const G=H.trim();G&&(g(r,G,!0),g(n,!1),l.runSearch(G))}function p(H){if(!a(n)||a(v).length===0){H.key==="Enter"&&h(a(r));return}H.key==="ArrowDown"?(H.preventDefault(),g(i,(a(i)+1)%a(v).length)):H.key==="ArrowUp"?(H.preventDefault(),g(i,(a(i)-1+a(v).length)%a(v).length)):H.key==="Enter"?(H.preventDefault(),h(a(i)>=0?a(v)[a(i)]:a(r))):H.key==="Escape"&&g(n,!1)}function b(H){const G=a(r).trim();return!a(c)&&G&&H.toLowerCase().startsWith(G.toLowerCase())?[H.slice(0,G.length),H.slice(G.length)]:["",H]}var y=Rc(),x=f(y),A=f(x),k=f(A);_o(k,{size:18});var P=u(x,2),E=f(P),W=f(E);ja(W,{size:15,class:"text-[var(--text-muted)] shrink-0"});var K=u(W,2),he=u(E,2);{var I=H=>{var G=Lc(),le=f(G),oe=f(le),ye=$(oe,!0),F=u(oe,2);{var ce=te=>{var se=jc();m("mousedown",se,()=>l.clearSearchHistory()),_(te,se)};z(F,te=>{a(c)&&te(ce)})}var D=u(le,2);Oe(D,17,()=>a(v),pr,(te,se,pe)=>{const ee=me(()=>{const[R,de]=b(a(se));return{prefix:R,rest:de}});var Ie=zc(),we=f(Ie);ja(we,{size:14,class:"text-[var(--text-muted)] shrink-0"});var Q=u(we,2),fe=f(Q);{var C=R=>{var de=qc(),Y=ke(de),U=$(Y,!0),ne=u(Y),ve=$(ne,!0);L(()=>{S(U,a(ee).prefix),S(ve,a(ee).rest)}),_(R,de)},O=R=>{var de=vr();L(()=>S(de,a(se))),_(R,de)};z(fe,R=>{a(ee).prefix?R(C):R(O,-1)})}L(()=>re(Ie,1,`w-full flex items-center gap-2.5 px-3.5 py-2 text-left text-[13.5px] transition-colors ${pe===a(i)?"bg-[var(--accent-soft)] text-[var(--accent-bright)]":"hover:bg-[var(--surface-hover)]"}`)),m("mousedown",Ie,()=>h(a(se))),_(te,Ie)}),L(()=>S(ye,a(c)?"Recent searches":"Suggestions")),_(H,G)};z(he,H=>{a(n)&&a(v).length>0&&H(I)})}var M=u(P,2),Z=f(M);let ue;var j=f(Z);wo(j,{size:18});var ie=u(Z,2);let N;var ae=f(ie);si(ae,{size:18});var T=u(ie,4);let B;var J=f(T);yo(J,{size:18}),L(()=>{ue=re(Z,1,"icon-btn",null,ue,{active:l.spinOpen||l.spinStatus.local_active||l.spinStatus.remote_role!=="idle"}),N=re(ie,1,"icon-btn",null,N,{active:l.partyMode}),B=re(T,1,"icon-btn",null,B,{active:l.settingsOpen})}),m("click",A,()=>{l.sidebarOpen=!l.sidebarOpen,l.persistSession()}),Ct("focus",K,()=>{a(v).length>0&&g(n,!0)}),Ct("blur",K,()=>window.setTimeout(()=>g(n,!1),140)),m("input",K,d),m("keydown",K,p),vs(K,()=>a(r),H=>g(r,H)),m("click",Z,()=>{l.spinOpen=!l.spinOpen,l.spinOpen&&l.refreshSpinStatus()}),m("click",ie,()=>l.partyMode=!l.partyMode),m("click",T,()=>l.settingsOpen=!0),_(t,y),ht()}kt(["click","input","keydown","mousedown"]);var Oc=w('<button><!> <div class="track-meta"><span class="track-title svelte-oxdkf2"> </span> <span class="track-sub"> </span></div> <span class="play-badge svelte-oxdkf2" aria-hidden="true"> </span></button>'),Fc=w('<section class="section svelte-oxdkf2"><h2 class="section-title">Listen again</h2> <div class="listen-grid svelte-oxdkf2"></div></section>'),Dc=w('<section class="section svelte-oxdkf2"><div class="hero-card svelte-oxdkf2"><!> <div class="hero-info svelte-oxdkf2"><span class="hero-badge svelte-oxdkf2">FEATURED</span> <h2 class="hero-title svelte-oxdkf2"> </h2> <p class="muted"> </p> <div class="hero-actions svelte-oxdkf2"><button class="pill-btn primary">▶ Play</button> <button class="pill-btn">Open album</button></div></div></div></section>'),Vc=w('<span class="year-badge numeric svelte-oxdkf2"> </span>'),Hc=w('<button class="media-card svelte-oxdkf2"><div class="media-cover-wrap svelte-oxdkf2"><!> <!></div> <strong class="media-title svelte-oxdkf2"> </strong> <span class="media-sub muted svelte-oxdkf2"> </span></button>'),Qc=w('<section class="section svelte-oxdkf2" id="new-releases"><h2 class="section-title">New releases</h2> <div class="rail-scroll"></div></section>'),Uc=w('<div role="button" tabindex="0"><span class="rank numeric svelte-oxdkf2"> </span> <!> <div class="track-meta"><span class="track-title"> </span> <span class="track-sub"> </span></div> <span class="muted numeric dur svelte-oxdkf2"> </span> <button title="Favorite">♥</button> <button class="icon-btn" title="Add to queue">+</button></div>'),Wc=w('<section class="section svelte-oxdkf2" id="trending-hits"><div class="section-head svelte-oxdkf2"><h2 class="section-title">Trending hits</h2> <div class="pill-group svelte-oxdkf2"><button>Trending</button> <button>Top Music</button></div></div> <div class="track-list"></div></section>'),Bc=w('<div class="empty-home svelte-oxdkf2"><p class="muted">Connect Tauri runtime (`cargo tauri dev`) to load live YouTube Music feeds.</p></div>'),Gc=w('<div class="page-clamp"><div class="home-head svelte-oxdkf2"><div><h1 class="greeting svelte-oxdkf2"> </h1> <p class="muted caption svelte-oxdkf2">Your music ready to play</p></div> <button class="icon-btn" title="Reload home">↻</button></div> <section class="quick-grid svelte-oxdkf2"><button class="quick-card q1 svelte-oxdkf2"><span class="q-badge svelte-oxdkf2">◎</span> <span class="q-text svelte-oxdkf2"><strong class="svelte-oxdkf2">New releases</strong><small class="muted svelte-oxdkf2">Recent albums &amp; singles</small></span></button> <button class="quick-card q2 svelte-oxdkf2"><span class="q-badge svelte-oxdkf2">☰</span> <span class="q-text svelte-oxdkf2"><strong class="svelte-oxdkf2">Top charts</strong><small class="muted svelte-oxdkf2">Trending &amp; Top Music</small></span></button> <button class="quick-card q3 svelte-oxdkf2"><span class="q-badge svelte-oxdkf2">⌕</span> <span class="q-text svelte-oxdkf2"><strong class="svelte-oxdkf2">Moods</strong><small class="muted svelte-oxdkf2">Explore genres &amp; vibes</small></span></button> <button class="quick-card q4 svelte-oxdkf2"><span class="q-badge svelte-oxdkf2">♥</span> <span class="q-text svelte-oxdkf2"><strong class="svelte-oxdkf2">Collection</strong><small class="muted svelte-oxdkf2">Your favorite songs</small></span></button></section> <!> <!> <!> <!></div>');function Yc(t,e){ft(e,!0);let r=q("trending"),s=me(()=>{const N=new Date().getHours();return N>=5&&N<12?"Good morning":N>=12&&N<19?"Good afternoon":"Good evening"}),n=me(()=>l.recentTracks.slice(0,6)),i=me(()=>l.homeFeed?.new_albums[0]??null),o=me(()=>l.homeFeed?l.homeFeed.new_albums.slice(l.homeFeed.new_albums.length>1?1:0,16):[]),c=me(()=>a(r)==="top"?l.homeFeed?.top??[]:l.homeFeed?.trending??[]);var v=Gc(),d=f(v),h=f(d),p=f(h),b=$(p,!0),y=u(h,2),x=u(d,2),A=f(x),k=u(A,2),P=u(k,2),E=u(P,2),W=u(x,2);{var K=N=>{var ae=Fc(),T=u(f(ae),2);Oe(T,23,()=>a(n),B=>B.id,(B,J,H)=>{const G=me(()=>l.currentTrack?.id===a(J).id);var le=Oc();let oe;var ye=f(le);Ge(ye,{class:"tile-cover",get src(){return a(J).cover_url},alt:""});var F=u(ye,2),ce=f(F),D=$(ce,!0),te=u(ce,2),se=$(te,!0),pe=u(F,2),ee=$(pe,!0);L(()=>{oe=re(le,1,"listen-tile svelte-oxdkf2",null,oe,{playing:a(G)}),S(D,a(J).title),S(se,a(J).artist),S(ee,a(G)?"🔊":"▶")}),m("click",le,()=>l.replaceQueueAndPlay(a(n),a(H),"Listen again")),_(B,le)}),_(N,ae)};z(W,N=>{a(n).length>0&&N(K)})}var he=u(W,2);{var I=N=>{var ae=Dc(),T=f(ae),B=f(T);Ge(B,{class:"hero-cover",get src(){return a(i).cover_url},get alt(){return a(i).title}});var J=u(B,2),H=u(f(J),2),G=$(H,!0),le=u(H,2),oe=$(le,!0),ye=u(le,2),F=f(ye),ce=u(F,2);L(()=>{S(G,a(i).title),S(oe,a(i).artist)}),m("click",F,()=>l.openAlbum(a(i).id,a(i).title,a(i).cover_url)),m("click",ce,()=>l.openAlbum(a(i).id,a(i).title,a(i).cover_url)),_(N,ae)};z(he,N=>{a(i)&&N(I)})}var M=u(he,2);{var Z=N=>{var ae=Qc(),T=u(f(ae),2);Oe(T,21,()=>a(o),B=>B.id,(B,J)=>{var H=Hc(),G=f(H),le=f(G);Ge(le,{class:"media-cover",get src(){return a(J).cover_url},get alt(){return a(J).title}});var oe=u(le,2);{var ye=se=>{var pe=Vc(),ee=$(pe,!0);L(()=>S(ee,a(J).year)),_(se,pe)};z(oe,se=>{a(J).year&&se(ye)})}var F=u(G,2),ce=$(F,!0),D=u(F,2),te=$(D,!0);L(()=>{S(ce,a(J).title),S(te,a(J).artist)}),m("click",H,()=>l.openAlbum(a(J).id,a(J).title,a(J).cover_url)),_(B,H)}),_(N,ae)};z(M,N=>{a(o).length>0&&N(Z)})}var ue=u(M,2);{var j=N=>{var ae=Wc(),T=f(ae),B=u(f(T),2),J=f(B);let H;var G=u(J,2);let le;var oe=u(T,2);Oe(oe,23,()=>a(c),ye=>ye.id,(ye,F,ce)=>{const D=me(()=>l.currentTrack?.id===a(F).id);var te=Uc();let se;var pe=f(te),ee=$(pe,!0),Ie=u(pe,2);Ge(Ie,{class:"track-cover",get src(){return a(F).cover_url},alt:""});var we=u(Ie,2),Q=f(we),fe=$(Q,!0),C=u(Q,2),O=$(C,!0),R=u(we,2),de=$(R,!0),Y=u(R,2);let U;var ne=u(Y,2);L((ve,xe,_e)=>{se=re(te,1,"track-row",null,se,{active:a(D)}),S(ee,ve),S(fe,a(F).title),S(O,a(F).artist),S(de,xe),U=re(Y,1,"icon-btn",null,U,{"fav-active":_e})},[()=>a(D)?"🔊":String(a(ce)+1).padStart(2,"0"),()=>dt(a(F).duration_secs),()=>l.isFavorite(a(F).id)]),m("click",te,()=>l.replaceQueueAndPlay(a(c),a(ce),"Trending hits")),m("keydown",te,ve=>ve.key==="Enter"&&l.replaceQueueAndPlay(a(c),a(ce),"Trending hits")),m("click",Y,ve=>{ve.stopPropagation(),l.toggleFavorite(a(F))}),m("click",ne,ve=>{ve.stopPropagation(),l.enqueueTrack(a(F))}),_(ye,te)}),L(()=>{H=re(J,1,"svelte-oxdkf2",null,H,{active:a(r)==="trending"}),le=re(G,1,"svelte-oxdkf2",null,le,{active:a(r)==="top"})}),m("click",J,()=>g(r,"trending")),m("click",G,()=>g(r,"top")),_(N,ae)},ie=N=>{var ae=Bc();_(N,ae)};z(ue,N=>{a(c).length>0?N(j):l.homeLoading||N(ie,1)})}L(()=>S(b,a(s))),m("click",y,()=>void l.loadHome(!0)),m("click",A,()=>document.getElementById("new-releases")?.scrollIntoView({behavior:"smooth"})),m("click",k,()=>document.getElementById("trending-hits")?.scrollIntoView({behavior:"smooth"})),m("click",P,()=>l.navigateTab("explore")),m("click",E,()=>l.navigateTab("collection")),_(t,v),ht()}kt(["click","keydown"]);var Kc=w('<div class="up-next svelte-1o5omgn"><span class="up-next-kicker svelte-1o5omgn">UP NEXT</span> <div class="up-next-row svelte-1o5omgn"><!> <div class="track-meta"><span class="track-title"> </span> <span class="track-sub"> </span></div></div></div>'),Jc=w('<span class="sync-pill svelte-1o5omgn">● Synced</span>'),Xc=w("<button> </button>"),Zc=w('<pre class="party-plain svelte-1o5omgn"> </pre>'),eu=w('<p class="muted">Play a song to view its lyrics.</p>'),tu=w('<div class="party-root svelte-1o5omgn"><div class="party-top svelte-1o5omgn"><div class="party-badge-row svelte-1o5omgn"><span class="party-pill svelte-1o5omgn">✦ Party Mode</span> <span class="muted"> </span></div> <button class="pill-btn">Exit</button></div> <div class="party-split svelte-1o5omgn"><div class="party-left svelte-1o5omgn"><!> <h1 class="party-title svelte-1o5omgn"> </h1> <p class="party-artist muted svelte-1o5omgn"> </p> <div class="party-progress-wrap svelte-1o5omgn"><div class="party-progress-bar svelte-1o5omgn"><div class="party-progress-fill svelte-1o5omgn"></div></div> <div class="party-times numeric svelte-1o5omgn"><span class="muted"> </span> <button title="Favorite">♥</button> <span class="muted"> </span></div></div> <div class="party-controls svelte-1o5omgn"><button>⇄</button> <button class="icon-btn">⏮</button> <button class="play-circle"><!></button> <button class="icon-btn">⏭</button> <button>↻</button></div> <!></div> <div class="party-right svelte-1o5omgn"><div class="party-lyrics-head svelte-1o5omgn"><h2 class="svelte-1o5omgn">Lyrics</h2> <!></div> <div class="party-lyrics-scroll svelte-1o5omgn"><!></div></div></div></div>');function ru(t,e){ft(e,!0);let r=Qe({});Ut(()=>{const C=l.activeLyricIndex;C!=null&&l.partyMode&&r[C]?.scrollIntoView({behavior:"smooth",block:"center"})});function s(C){if(l.activeLyricIndex==null)return .72;if(C===l.activeLyricIndex)return 1;const O=Math.abs(C-l.activeLyricIndex);return O===1?.66:O===2?.42:.24}var n=tu(),i=f(n),o=f(i),c=u(f(o),2),v=$(c,!0),d=u(o,2),h=u(i,2),p=f(h),b=f(p);{let C=me(()=>l.currentTrack?.cover_large_url),O=me(()=>l.currentTrack?.cover_url),R=me(()=>l.currentTrack?.title??"");Ge(b,{class:"party-cover",get src(){return a(C)},get fallbackSrc(){return a(O)},get alt(){return a(R)}})}var y=u(b,2),x=$(y,!0),A=u(y,2),k=$(A,!0),P=u(A,2),E=f(P),W=$(E),K=u(E,2),he=f(K),I=$(he,!0),M=u(he,2);let Z;var ue=u(M,2),j=$(ue,!0),ie=u(P,2),N=f(ie);let ae;var T=u(N,2),B=u(T,2),J=f(B);{var H=C=>{var O=vr("❚❚");_(C,O)},G=C=>{var O=vr("▶");_(C,O)};z(J,C=>{l.isPlaying?C(H):C(G,-1)})}var le=u(B,2),oe=u(le,2);let ye;var F=u(ie,2);{var ce=C=>{var O=Kc(),R=u(f(O),2),de=f(R);Ge(de,{class:"up-next-cover",get src(){return l.nextTrack.cover_url},alt:""});var Y=u(de,2),U=f(Y),ne=$(U,!0),ve=u(U,2),xe=$(ve,!0);L(()=>{S(ne,l.nextTrack.title),S(xe,l.nextTrack.artist)}),_(C,O)};z(F,C=>{l.nextTrack&&C(ce)})}var D=u(p,2),te=f(D),se=u(f(te),2);{var pe=C=>{var O=Jc();_(C,O)};z(se,C=>{l.lyrics.type==="Synced"&&C(pe)})}var ee=u(te,2),Ie=f(ee);{var we=C=>{var O=Ne(),R=ke(O);Oe(R,17,()=>l.lyrics.lines,pr,(de,Y,U)=>{var ne=Xc();let ve;var xe=$(ne,!0);ms(ne,(_e,be)=>r[be]=_e,_e=>r?.[_e],()=>[U]),L(_e=>{ve=re(ne,1,"party-line svelte-1o5omgn",null,ve,{active:l.activeLyricIndex===U}),bt(ne,`opacity: ${_e??""}`),S(xe,a(Y).text)},[()=>s(U)]),m("click",ne,()=>l.seek(a(Y).time_secs)),_(de,ne)}),_(C,O)},Q=C=>{var O=Zc(),R=$(O,!0);L(()=>S(R,l.lyrics.text)),_(C,O)},fe=C=>{var O=eu();_(C,O)};z(Ie,C=>{l.lyrics.type==="Synced"?C(we):l.lyrics.type==="Plain"?C(Q,1):C(fe,-1)})}L((C,O,R,de)=>{S(v,l.sourceName),S(x,l.currentTrack?.title??"Nothing playing"),S(k,l.currentTrack?.artist??""),bt(W,`width: ${C??""}%`),S(I,O),Z=re(M,1,"icon-btn",null,Z,{"fav-active":R}),M.disabled=!l.currentTrack,S(j,de),ae=re(N,1,"icon-btn",null,ae,{active:l.shuffle}),ye=re(oe,1,"icon-btn",null,ye,{active:l.repeat})},[()=>l.durationSecs>0?Math.min(100,l.positionSecs/l.durationSecs*100):0,()=>dt(l.positionSecs),()=>l.currentTrack?l.isFavorite(l.currentTrack.id):!1,()=>dt(l.durationSecs)]),m("click",d,()=>l.partyMode=!1),m("click",M,()=>l.currentTrack&&l.toggleFavorite(l.currentTrack)),m("click",N,()=>l.toggleShuffle()),m("click",T,()=>void l.prev()),m("click",B,()=>void l.togglePlay()),m("click",le,()=>void l.next()),m("click",oe,()=>l.toggleRepeat()),_(t,n),ht()}kt(["click"]);var su=w('<dialog class="gt-dialog"><div class="dlg-head svelte-qv6s6m"><h2 class="svelte-qv6s6m">Settings</h2> <button class="icon-btn">✕</button></div> <div class="dlg-body svelte-qv6s6m"><section class="group svelte-qv6s6m"><h3 class="svelte-qv6s6m">Appearance</h3> <div class="card svelte-qv6s6m"><div class="row svelte-qv6s6m"><span>Theme</span> <select class="svelte-qv6s6m"><option>System</option><option>Dark</option><option>Light</option></select></div></div></section> <section class="group svelte-qv6s6m"><h3 class="svelte-qv6s6m">Playback</h3> <div class="card svelte-qv6s6m"><div class="row svelte-qv6s6m"><div class="svelte-qv6s6m"><strong>Crossfade</strong> <small class="muted svelte-qv6s6m"> </small></div> <input type="range" min="0" max="10" step="1"/></div> <label class="row svelte-qv6s6m"><div class="svelte-qv6s6m"><strong>Autoplay</strong> <small class="muted svelte-qv6s6m">Queue similar songs when the queue ends</small></div> <input type="checkbox"/></label> <label class="row svelte-qv6s6m"><div class="svelte-qv6s6m"><strong>Background playback</strong> <small class="muted svelte-qv6s6m">Keep playing music when closing the window</small></div> <input type="checkbox"/></label></div></section> <section class="group svelte-qv6s6m"><h3 class="svelte-qv6s6m">Storage</h3> <div class="card svelte-qv6s6m"><div class="row svelte-qv6s6m"><div class="svelte-qv6s6m"><strong>Audio cache</strong> <small class="muted numeric svelte-qv6s6m"> </small> <div class="cache-bar svelte-qv6s6m"><div class="cache-fill svelte-qv6s6m"></div></div></div> <button class="pill-btn danger svelte-qv6s6m">Clear</button></div> <div class="row svelte-qv6s6m"><div class="svelte-qv6s6m"><strong>Search history</strong> <small class="muted svelte-qv6s6m"> </small></div> <button class="pill-btn">Clear</button></div></div></section></div></dialog>');function au(t,e){ft(e,!0);let r=q(null),s=q(0);Ut(()=>{const ee=a(r);ee&&(l.settingsOpen&&!ee.open?(ee.showModal(),Le()&&Ae.getCacheBytes().then(Ie=>g(s,Ie,!0)).catch(()=>{})):!l.settingsOpen&&ee.open&&ee.close())});let n=me(()=>(a(s)/(1024*1024)).toFixed(1)),i=me(()=>Math.min(100,a(s)/(1024*1024*1024)*100));async function o(){Le()&&(await Ae.clearAudioCache(),g(s,0),l.showToast("Audio cache cleared"))}var c=su(),v=f(c),d=u(f(v),2),h=u(v,2),p=f(h),b=u(f(p),2),y=f(b),x=u(f(y),2),A=f(x);A.value=A.__value="system";var k=u(A);k.value=k.__value="dark";var P=u(k);P.value=P.__value="light";var E;Wn(x);var W=u(p,2),K=u(f(W),2),he=f(K),I=f(he),M=u(f(I),2),Z=$(M,!0),ue=u(I,2),j=u(he,2),ie=u(f(j),2),N=u(j,2),ae=u(f(N),2),T=u(W,2),B=u(f(T),2),J=f(B),H=f(J),G=u(f(H),2),le=$(G),oe=u(G,2),ye=$(oe),F=u(H,2),ce=u(J,2),D=f(ce),te=u(f(D),2),se=$(te),pe=u(D,2);ms(c,ee=>g(r,ee),()=>a(r)),L(()=>{E!==(E=l.themeMode)&&(x.value=(x.__value=E)??"",us(x,E)),S(Z,l.crossfadeSecs===0?"Disabled (0 s)":`${l.crossfadeSecs} s`),Gs(ue,l.crossfadeSecs),Ur(ie,l.autoplayRelated),Ur(ae,l.backgroundPlayback),S(le,`${a(n)??""} MB / 1024 MB cached`),bt(ye,`width: ${a(i)??""}%`),F.disabled=a(s)===0,S(se,`${l.searchHistory.length??""} saved searches`),pe.disabled=l.searchHistory.length===0}),Ct("close",c,()=>l.settingsOpen=!1),m("click",c,ee=>{ee.target===a(r)&&(l.settingsOpen=!1)}),m("click",d,()=>l.settingsOpen=!1),m("change",x,ee=>l.setThemeMode(ee.currentTarget.value)),m("input",ue,ee=>l.setCrossfadeSecs(Number(ee.currentTarget.value))),m("change",ie,ee=>l.setAutoplay(ee.currentTarget.checked)),m("change",ae,ee=>l.setBackgroundPlayback(ee.currentTarget.checked)),m("click",F,()=>void o()),m("click",pe,()=>l.clearSearchHistory()),_(t,c),ht()}kt(["click","change","input"]);var nu=w('<p class="text-[12px] text-[var(--text-muted)]/75 truncate svelte-1jr4poh"> </p>'),iu=w('<span class="w-4 h-4 rounded-full border-2 border-current/30 border-t-current animate-spin svelte-1jr4poh"></span>'),lu=w('<span class="w-1.5 h-1.5 rounded-full bg-[var(--accent-bright)] absolute top-1 right-1 svelte-1jr4poh"></span>'),ou=w('<button type="button" aria-label="Close popover" class="fixed inset-0 z-20 cursor-default bg-transparent svelte-1jr4poh"></button>'),cu=w('<div class="absolute bottom-12 left-1/2 -translate-x-1/2 w-[272px] p-3.5 rounded-2xl bg-[var(--bg-elevated)]/95 backdrop-blur-md border border-[var(--border-strong)] shadow-[0_16px_40px_rgba(0,0,0,0.5)] z-30 space-y-2.5 svelte-1jr4poh"><div class="flex items-center justify-between text-[12px] svelte-1jr4poh"><span class="font-semibold text-[var(--text-muted)] svelte-1jr4poh">Volume</span> <span class="font-bold text-[var(--accent-bright)] numeric svelte-1jr4poh"> </span></div> <div class="flex items-center gap-3 svelte-1jr4poh"><button><!></button> <input class="sidebar-seek flex-1 min-w-0 svelte-1jr4poh" type="range" min="0" max="1" step="0.01" aria-label="Volume"/></div></div>'),uu=w('<span class="font-bold text-[var(--accent-bright)] numeric svelte-1jr4poh"> </span>'),vu=w('<button class="py-1.5 rounded-xl text-[12px] font-semibold bg-[var(--surface)] text-[var(--text-muted)] hover:bg-[var(--accent-soft)] hover:text-[var(--accent-bright)] transition-colors svelte-1jr4poh"> </button>'),du=w('<div class="absolute bottom-12 left-1/2 -translate-x-1/2 w-[272px] p-3.5 rounded-2xl bg-[var(--bg-elevated)]/95 backdrop-blur-md border border-[var(--border-strong)] shadow-[0_16px_40px_rgba(0,0,0,0.5)] z-30 space-y-2.5 svelte-1jr4poh"><div class="flex items-center justify-between text-[12px] svelte-1jr4poh"><span class="font-semibold text-[var(--text-muted)] svelte-1jr4poh">Sleep timer</span> <!></div> <div class="grid grid-cols-5 gap-1.5 svelte-1jr4poh"></div></div>'),Ha=w('<span class="w-1 h-1 rounded-full bg-[var(--accent-bright)] absolute bottom-0.5 svelte-1jr4poh"></span>'),fu=w('<span class="w-5 h-5 rounded-full border-2 border-white/30 border-t-white animate-spin svelte-1jr4poh"></span>'),hu=w('<div class="flex-1 flex flex-col items-center justify-between py-4 gap-5 svelte-1jr4poh"><div class="flex-1 flex items-center justify-center w-full py-2 svelte-1jr4poh"><!></div> <div class="w-full flex flex-col items-center gap-4 svelte-1jr4poh"><div class="w-full text-center px-1 space-y-1 svelte-1jr4poh"><h2 class="text-[19px] font-extrabold leading-tight tracking-tight line-clamp-2 svelte-1jr4poh"> </h2> <p class="text-[14px] font-medium text-[var(--text-muted)] truncate svelte-1jr4poh"> </p> <!></div> <div class="relative w-full flex items-center justify-center gap-5 py-0.5 svelte-1jr4poh"><button><!></button> <button><!></button> <button title="Start radio from this song"><!></button> <button><!> <!></button> <button title="Party Mode"><!></button> <!> <!></div> <div class="w-full space-y-2.5 svelte-1jr4poh"><input class="sidebar-seek w-full svelte-1jr4poh" type="range" min="0" step="0.5" aria-label="Seek"/> <div class="flex items-center justify-between text-[12px] text-[var(--text-muted)] numeric svelte-1jr4poh"><span class="svelte-1jr4poh"> </span> <span class="px-3 py-0.5 rounded-full text-[11px] font-bold bg-[var(--accent-soft)] text-[var(--accent-bright)] svelte-1jr4poh"> </span> <span class="svelte-1jr4poh"> </span></div></div> <div class="w-full flex items-center justify-center gap-4 pt-1 pb-1 svelte-1jr4poh"><button title="Shuffle"><!> <!></button> <button class="icon-btn !w-9 !h-9 svelte-1jr4poh" title="Previous"><!></button> <button class="play-circle !w-[56px] !h-[56px] shrink-0 svelte-1jr4poh" title="Play / Pause"><!></button> <button class="icon-btn !w-9 !h-9 svelte-1jr4poh" title="Next"><!></button> <button title="Repeat"><!> <!></button></div></div></div>'),pu=w('<p class="text-[12px] text-[var(--text-muted)] truncate svelte-1jr4poh"> </p>'),_u=w('<span class="shrink-0 inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full text-[10.5px] font-bold bg-[var(--accent-soft)] text-[var(--accent-bright)] svelte-1jr4poh"><span class="w-1.5 h-1.5 rounded-full bg-[var(--accent-bright)] svelte-1jr4poh"></span> Synced</span>'),gu=w('<span class="shrink-0 px-2.5 py-0.5 rounded-full text-[10.5px] font-semibold bg-[var(--surface)] text-[var(--text-muted)] svelte-1jr4poh">Static</span>'),bu=w("<button> </button>"),mu=w('<div class="lyrics-fade flex flex-col gap-1 py-4 svelte-1jr4poh"></div>'),yu=w('<pre class="whitespace-pre-wrap font-sans text-[14px] leading-relaxed text-[var(--text)]/85 py-2 svelte-1jr4poh"> </pre>'),xu=w('<div class="flex-1 flex flex-col items-center justify-center text-center gap-2 py-12 text-[var(--text-muted)] svelte-1jr4poh"><!> <p class="text-[13px] svelte-1jr4poh"> </p></div>'),ku=w('<div class="flex flex-col gap-3 pt-1 flex-1 min-h-0 svelte-1jr4poh"><div class="flex items-start justify-between gap-2 pb-2 border-b border-[var(--border)] svelte-1jr4poh"><div class="min-w-0 svelte-1jr4poh"><h3 class="text-[14.5px] font-bold truncate svelte-1jr4poh"> </h3> <!></div> <!></div> <!></div>'),wu=w('<button class="px-2.5 py-1 rounded-full text-[11.5px] font-semibold bg-[var(--surface)] hover:bg-red-500/20 hover:text-red-400 transition-colors svelte-1jr4poh">Clear</button>'),Su=w('<div class="flex flex-col items-center justify-center text-center gap-2 py-10 text-[var(--text-muted)] svelte-1jr4poh"><!> <p class="text-[13px] svelte-1jr4poh">The queue is empty.</p></div>'),$u=w('<span class="eq-bars shrink-0 svelte-1jr4poh" aria-label="Playing"><span class="svelte-1jr4poh"></span><span class="svelte-1jr4poh"></span><span class="svelte-1jr4poh"></span></span>'),Tu=w('<span class="text-[11px] text-[var(--text-muted)] numeric shrink-0 svelte-1jr4poh"> </span>'),Pu=w('<div draggable="true" role="button" tabindex="0"><!> <!> <div class="track-meta svelte-1jr4poh"><span> </span> <span class="track-sub !text-[11px] svelte-1jr4poh"> </span></div> <!> <button class="icon-btn !w-6 !h-6 opacity-0 group-hover:opacity-100 transition-opacity shrink-0 svelte-1jr4poh" title="Remove from queue"><!></button></div>'),Au=w('<div class="track-list svelte-1jr4poh"></div>'),Eu=w('<div class="flex items-center gap-2.5 px-2.5 py-2 border-b border-[var(--border)] last:border-b-0 hover:bg-[var(--surface-hover)] transition-colors svelte-1jr4poh"><!> <div class="track-meta svelte-1jr4poh"><span class="track-title !text-[12.5px] svelte-1jr4poh"> </span> <span class="track-sub !text-[11px] svelte-1jr4poh"> </span></div> <button class="icon-btn !w-7 !h-7 shrink-0 svelte-1jr4poh" title="Add to queue"><!></button></div>'),Iu=w('<h4 class="text-[11.5px] font-bold uppercase tracking-wider text-[var(--text-muted)] mt-1 svelte-1jr4poh">Up next suggestions</h4> <div class="track-list svelte-1jr4poh"></div>',1),Mu=w('<div class="flex flex-col gap-3 pt-1 svelte-1jr4poh"><div class="flex items-center justify-between gap-2 svelte-1jr4poh"><h3 class="text-[14px] font-bold svelte-1jr4poh">Queue <span class="text-[var(--text-muted)] font-medium svelte-1jr4poh"> </span></h3> <!></div> <!> <div class="flex items-center justify-between gap-3 p-3 rounded-xl bg-[var(--surface)] border border-[var(--border)] svelte-1jr4poh"><div class="min-w-0 svelte-1jr4poh"><div class="text-[13px] font-bold svelte-1jr4poh">Autoplay</div> <p class="text-[11.5px] text-[var(--text-muted)] leading-snug svelte-1jr4poh">Queue similar songs when queue ends</p></div> <button type="button" role="switch" aria-label="Toggle autoplay"><span></span></button></div> <!></div>'),Nu=w('<aside class="w-[388px] shrink-0 h-full flex flex-col border-r border-[var(--border)] bg-[var(--bg-sidebar)] overflow-hidden relative select-none svelte-1jr4poh" style="background-image: linear-gradient(180deg, rgba(var(--ambient-rgb), 0.22) 0%, rgba(var(--ambient-rgb), 0.08) 52%, transparent 100%);"><div class="shrink-0 flex items-center justify-between gap-2 px-6 pt-4 pb-2 svelte-1jr4poh"><div class="min-w-0 flex flex-col leading-tight svelte-1jr4poh"><span class="text-[11.5px] font-medium text-[var(--text-muted)] svelte-1jr4poh">Playing from</span> <span class="text-[13px] font-semibold truncate svelte-1jr4poh"> </span></div> <button class="icon-btn shrink-0 text-[var(--text-muted)] hover:text-[var(--text)] svelte-1jr4poh" title="Collapse player"><!></button></div> <div class="flex-1 min-h-0 overflow-y-auto scroll-y px-7 pb-4 flex flex-col svelte-1jr4poh"><!></div> <nav class="shrink-0 grid grid-cols-3 gap-1.5 px-3 py-2.5 border-t border-[var(--border)] bg-[var(--bg-sidebar)] svelte-1jr4poh"><button><!> Player</button> <button><!> Lyrics</button> <button><!> Queue</button></nav></aside>');function ju(t,e){ft(e,!0);let r=Qe({}),s=q(null),n=q(null),i=q(!1),o=q(0),c=q(null),v=q(1),d=q(!1),h=q(null),p,b=me(()=>a(i)?a(o):l.positionSecs),y=me(()=>l.durationSecs>0?Math.max(0,Math.min(100,a(b)/l.durationSecs*100)):0),x=me(()=>Math.round(l.volume*100)),A=me(()=>l.currentTrack?l.isFavorite(l.currentTrack.id):!1);Ut(()=>{const F=l.activeLyricIndex;F!=null&&l.sidebarTab==="lyrics"&&r[F]?.scrollIntoView({behavior:"smooth",block:"center"})});function k(F){g(c,a(c)===F?null:F,!0)}function P(){l.volume>.01?(g(v,l.volume,!0),l.setVolume(0)):l.setVolume(a(v)>.05?a(v):.8)}function E(F){if(window.clearInterval(p),p=void 0,!F){g(h,null),g(c,null);return}g(h,F*60),g(c,null),l.showToast(`Sleep timer set for ${F} min`),p=window.setInterval(()=>{a(h)!=null&&(g(h,a(h)-1),a(h)<=0&&(window.clearInterval(p),p=void 0,g(h,null),l.isPlaying&&l.togglePlay(),l.showToast("Sleep timer ended — playback paused")))},1e3)}var W=Nu(),K=f(W),he=f(K),I=u(f(he),2),M=$(I,!0),Z=u(he,2),ue=f(Z);co(ue,{size:19});var j=u(K,2),ie=f(j);{var N=F=>{var ce=hu(),D=f(ce),te=f(D);{let X=me(()=>l.currentTrack?.cover_large_url),Ce=me(()=>l.currentTrack?.cover_url),$t=me(()=>l.currentTrack?.title??"");Ge(te,{class:"cover-large !w-[264px] !h-[264px] !rounded-2xl border border-white/10 shadow-[0_22px_50px_rgba(0,0,0,0.48)]",get src(){return a(X)},get fallbackSrc(){return a(Ce)},get alt(){return a($t)}})}var se=u(D,2),pe=f(se),ee=f(pe),Ie=$(ee,!0),we=u(ee,2),Q=$(we,!0),fe=u(we,2);{var C=X=>{var Ce=nu(),$t=$(Ce);L(()=>S($t,`Album: ${l.currentTrack.album??""}`)),_(X,Ce)};z(fe,X=>{l.currentTrack?.album&&X(C)})}var O=u(pe,2),R=f(O),de=f(R);{var Y=X=>{La(X,{size:18})},U=X=>{qa(X,{size:18})},ne=X=>{za(X,{size:18})};z(de,X=>{l.volume===0?X(Y):l.volume<.5?X(U,1):X(ne,-1)})}var ve=u(R,2),xe=f(ve);{let X=me(()=>a(A)?"currentColor":"none");Xn(xe,{size:18,get fill(){return a(X)}})}var _e=u(ve,2),be=f(_e);{var V=X=>{var Ce=iu();_(X,Ce)},Te=X=>{bo(X,{size:18})};z(be,X=>{a(d)?X(V):X(Te,-1)})}var ge=u(_e,2),Me=f(ge);ko(Me,{size:18});var je=u(Me,2);{var We=X=>{var Ce=lu();_(X,Ce)};z(je,X=>{a(h)!=null&&X(We)})}var Pe=u(ge,2),Xe=f(Pe);si(Xe,{size:18});var at=u(Pe,2);{var lt=X=>{var Ce=ou();m("click",Ce,()=>g(c,null)),_(X,Ce)};z(at,X=>{a(c)!==null&&X(lt)})}var rt=u(at,2);{var zt=X=>{var Ce=cu(),$t=f(Ce),jr=u(f($t),2),ws=$(jr),Ss=u($t,2),Tt=f(Ss),Yt=f(Tt);{var ir=Pt=>{La(Pt,{size:16})},qr=Pt=>{qa(Pt,{size:16})},di=Pt=>{za(Pt,{size:16})};z(Yt,Pt=>{l.volume===0?Pt(ir):l.volume<.5?Pt(qr,1):Pt(di,-1)})}var $s=u(Tt,2);L(()=>{S(ws,`${a(x)??""}%`),re(Tt,1,`icon-btn !w-8 !h-8 shrink-0 bg-[var(--surface)] hover:bg-[var(--surface-hover)] ${l.volume===0?"text-[var(--coral)]":"text-[var(--text)]"}`,"svelte-1jr4poh"),_t(Tt,"title",l.volume===0?"Unmute":"Mute"),Gs($s,l.volume),bt($s,`background: linear-gradient(to right, var(--accent-bright) ${a(x)??""}%, rgba(255,255,255,0.16) ${a(x)??""}%);`)}),m("click",Tt,P),m("input",$s,Pt=>l.setVolume(Number(Pt.currentTarget.value))),_(X,Ce)},wt=X=>{var Ce=du(),$t=f(Ce),jr=u(f($t),2);{var ws=Tt=>{var Yt=uu(),ir=$(Yt,!0);L(qr=>S(ir,qr),[()=>dt(a(h))]),_(Tt,Yt)};z(jr,Tt=>{a(h)!=null&&Tt(ws)})}var Ss=u($t,2);Oe(Ss,20,()=>[null,15,30,45,60],pr,(Tt,Yt)=>{var ir=vu(),qr=$(ir,!0);L(()=>S(qr,Yt==null?"Off":`${Yt}m`)),m("click",ir,()=>E(Yt)),_(Tt,ir)}),_(X,Ce)};z(rt,X=>{a(c)==="volume"?X(zt):a(c)==="timer"&&X(wt,1)})}var Ue=u(O,2),Be=f(Ue),nt=u(Be,2),ot=f(nt),ar=$(ot,!0),nr=u(ot,2),St=$(nr,!0),Er=u(nr,2),ys=$(Er,!0),xs=u(Ue,2),Ke=f(xs),Lt=f(Ke);xo(Lt,{size:17});var Ir=u(Lt,2);{var Mr=X=>{var Ce=Ha();_(X,Ce)};z(Ir,X=>{l.shuffle&&X(Mr)})}var Gt=u(Ke,2),Nr=f(Gt);ti(Nr,{size:20,fill:"currentColor"});var _r=u(Gt,2),ni=f(_r);{var ii=X=>{var Ce=fu();_(X,Ce)},li=X=>{Zn(X,{size:22,fill:"currentColor"})},oi=X=>{ei(X,{size:22,fill:"currentColor",class:"translate-x-[1.5px]"})};z(ni,X=>{l.isLoadingAudio?X(ii):l.isPlaying?X(li,1):X(oi,-1)})}var es=u(_r,2),ci=f(es);ri(ci,{size:20,fill:"currentColor"});var ks=u(es,2),va=f(ks);mo(va,{size:17});var ui=u(va,2);{var vi=X=>{var Ce=Ha();_(X,Ce)};z(ui,X=>{l.repeat&&X(vi)})}L((X,Ce,$t,jr)=>{S(Ie,l.currentTrack?.title??"Nothing playing"),S(Q,l.currentTrack?.artist??"Select a song"),re(R,1,`icon-btn !w-9 !h-9 ${a(c)==="volume"||l.volume===0?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`,"svelte-1jr4poh"),_t(R,"title",`Volume (${a(x)??""}%)`),re(ve,1,`icon-btn !w-9 !h-9 transition-transform active:scale-90 ${a(A)?"text-[var(--coral)]":"text-[var(--text-muted)] hover:text-[var(--text)]"}`,"svelte-1jr4poh"),ve.disabled=!l.currentTrack,_t(ve,"title",a(A)?"Remove from favorites":"Add to favorites"),re(_e,1,`icon-btn !w-9 !h-9 ${a(d)?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`,"svelte-1jr4poh"),_e.disabled=!l.currentTrack||a(d),re(ge,1,`icon-btn !w-9 !h-9 relative ${a(c)==="timer"||a(h)!=null?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`,"svelte-1jr4poh"),_t(ge,"title",X),re(Pe,1,`icon-btn !w-9 !h-9 ${l.partyMode?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`,"svelte-1jr4poh"),_t(Be,"max",Ce),Gs(Be,a(b)),Be.disabled=!l.currentTrack,bt(Be,`background: linear-gradient(to right, var(--accent-bright) ${a(y)??""}%, rgba(255,255,255,0.16) ${a(y)??""}%);`),S(ar,$t),S(St,l.qualityLabel),S(ys,jr),re(Ke,1,`icon-btn relative ${l.shuffle?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`,"svelte-1jr4poh"),Gt.disabled=!l.currentTrack,_r.disabled=!l.currentTrack,es.disabled=!l.currentTrack,re(ks,1,`icon-btn relative ${l.repeat?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`,"svelte-1jr4poh")},[()=>a(h)!=null?`Sleep timer: ${dt(a(h))}`:"Sleep timer",()=>Math.max(1,l.durationSecs),()=>dt(a(b)),()=>dt(l.durationSecs)]),m("click",R,()=>k("volume")),m("click",ve,()=>l.currentTrack&&l.toggleFavorite(l.currentTrack)),m("click",_e,async()=>{g(c,null),g(d,!0);try{await l.startRadioFromCurrent()}finally{g(d,!1)}}),m("click",ge,()=>k("timer")),m("click",Pe,()=>{g(c,null),l.partyMode=!0}),m("pointerdown",Be,()=>{g(i,!0),g(o,l.positionSecs,!0)}),m("input",Be,X=>{g(o,Number(X.currentTarget.value),!0)}),m("change",Be,X=>{const Ce=Number(X.currentTarget.value);g(i,!1),l.seek(Ce)}),m("click",Ke,()=>l.toggleShuffle()),m("click",Gt,()=>void l.prev()),m("click",_r,()=>void l.togglePlay()),m("click",es,()=>void l.next()),m("click",ks,()=>l.toggleRepeat()),_(F,ce)},ae=F=>{var ce=ku(),D=f(ce),te=f(D),se=f(te),pe=$(se,!0),ee=u(se,2);{var Ie=Y=>{var U=pu(),ne=$(U,!0);L(()=>S(ne,l.currentTrack.artist)),_(Y,U)};z(ee,Y=>{l.currentTrack&&Y(Ie)})}var we=u(te,2);{var Q=Y=>{var U=_u();_(Y,U)},fe=Y=>{var U=gu();_(Y,U)};z(we,Y=>{l.lyrics.type==="Synced"?Y(Q):l.lyrics.type==="Plain"&&Y(fe,1)})}var C=u(D,2);{var O=Y=>{var U=mu();Oe(U,21,()=>l.lyrics.lines,pr,(ne,ve,xe)=>{var _e=bu();let be;var V=$(_e,!0);ms(_e,(Te,ge)=>r[ge]=Te,Te=>r?.[Te],()=>[xe]),L(()=>{be=re(_e,1,"lyric-line svelte-1jr4poh",null,be,{active:l.activeLyricIndex===xe}),S(V,a(ve).text)}),m("click",_e,()=>l.seek(a(ve).time_secs)),_(ne,_e)}),_(Y,U)},R=Y=>{var U=yu(),ne=$(U,!0);L(()=>S(ne,l.lyrics.text)),_(Y,U)},de=Y=>{var U=xu(),ne=f(U);Na(ne,{size:28,class:"opacity-40"});var ve=u(ne,2),xe=$(ve,!0);L(()=>S(xe,l.currentTrack?"No lyrics available for this track.":"Play a song to view its lyrics.")),_(Y,U)};z(C,Y=>{l.lyrics.type==="Synced"?Y(O):l.lyrics.type==="Plain"?Y(R,1):Y(de,-1)})}L(()=>S(pe,l.currentTrack?.title??"Lyrics")),_(F,ce)},T=F=>{var ce=Mu(),D=f(ce),te=f(D),se=u(f(te)),pe=$(se),ee=u(te,2);{var Ie=U=>{var ne=wu();m("click",ne,()=>l.clearQueue()),_(U,ne)};z(ee,U=>{l.queue.length>0&&U(Ie)})}var we=u(D,2);{var Q=U=>{var ne=Su(),ve=f(ne);Ma(ve,{size:26,class:"opacity-40"}),_(U,ne)},fe=U=>{var ne=Au();Oe(ne,23,()=>l.queue,(ve,xe)=>ve.id+":"+xe,(ve,xe,_e)=>{const be=me(()=>l.currentIndex===a(_e));var V=Pu(),Te=f(V);vo(Te,{size:13,class:"text-[var(--text-muted)]/45 cursor-grab shrink-0"});var ge=u(Te,2);Ge(ge,{class:"track-cover !w-9 !h-9 !rounded-md",get src(){return a(xe).cover_url},alt:""});var Me=u(ge,2),je=f(Me),We=$(je,!0),Pe=u(je,2),Xe=$(Pe,!0),at=u(Me,2);{var lt=Ue=>{var Be=$u();_(Ue,Be)},rt=Ue=>{var Be=Tu(),nt=$(Be,!0);L(ot=>S(nt,ot),[()=>dt(a(xe).duration_secs)]),_(Ue,Be)};z(at,Ue=>{a(be)&&l.isPlaying?Ue(lt):Ue(rt,-1)})}var zt=u(at,2),wt=f(zt);So(wt,{size:13}),L(()=>{re(V,1,`group flex items-center gap-2 px-2.5 py-2 border-b border-[var(--border)] last:border-b-0 cursor-pointer transition-colors ${a(be)?"bg-[var(--accent-soft)]":"hover:bg-[var(--surface-hover)]"} ${a(n)===a(_e)?"border-t-2 border-t-[var(--accent-bright)]":""}`,"svelte-1jr4poh"),re(je,1,`track-title !text-[12.5px] ${a(be)?"!text-[var(--accent-bright)] !font-bold":""}`,"svelte-1jr4poh"),S(We,a(xe).title),S(Xe,a(xe).artist)}),Ct("dragstart",V,()=>g(s,a(_e),!0)),Ct("dragover",V,Ue=>{Ue.preventDefault(),g(n,a(_e),!0)}),Ct("dragleave",V,()=>{a(n)===a(_e)&&g(n,null)}),Ct("drop",V,()=>{a(s)!=null&&l.moveQueueItem(a(s),a(_e)),g(s,null),g(n,null)}),Ct("dragend",V,()=>{g(s,null),g(n,null)}),m("click",V,()=>void l.playIndex(a(_e))),m("keydown",V,Ue=>Ue.key==="Enter"&&void l.playIndex(a(_e))),m("click",zt,Ue=>{Ue.stopPropagation(),l.removeQueueIndex(a(_e))}),_(ve,V)}),_(U,ne)};z(we,U=>{l.queue.length===0?U(Q):U(fe,-1)})}var C=u(we,2),O=u(f(C),2),R=$(O),de=u(C,2);{var Y=U=>{var ne=Iu(),ve=u(ke(ne),2);Oe(ve,21,()=>l.suggestedTracks,xe=>xe.id,(xe,_e)=>{var be=Eu(),V=f(be);Ge(V,{class:"track-cover !w-9 !h-9 !rounded-md",get src(){return a(_e).cover_url},alt:""});var Te=u(V,2),ge=f(Te),Me=$(ge,!0),je=u(ge,2),We=$(je,!0),Pe=u(Te,2),Xe=f(Pe);go(Xe,{size:15}),L(()=>{S(Me,a(_e).title),S(We,a(_e).artist)}),m("click",Pe,()=>l.enqueueTrack(a(_e))),_(xe,be)}),_(U,ne)};z(de,U=>{l.suggestedTracks.length>0&&U(Y)})}L(()=>{S(pe,`· ${l.queue.length??""}`),_t(O,"aria-checked",l.autoplayRelated),re(O,1,`w-10 h-6 rounded-full p-0.5 transition-colors shrink-0 ${l.autoplayRelated?"bg-[var(--accent)]":"bg-white/15"}`,"svelte-1jr4poh"),re(R,1,`block w-5 h-5 rounded-full bg-white shadow transition-transform ${l.autoplayRelated?"translate-x-4":"translate-x-0"}`,"svelte-1jr4poh")}),m("click",O,()=>l.setAutoplay(!l.autoplayRelated)),_(F,ce)};z(ie,F=>{l.sidebarTab==="player"?F(N):l.sidebarTab==="lyrics"?F(ae,1):F(T,-1)})}var B=u(j,2),J=f(B),H=f(J);Na(H,{size:15});var G=u(J,2),le=f(G);po(le,{size:15});var oe=u(G,2),ye=f(oe);Ma(ye,{size:15}),L(()=>{S(M,l.sourceName),re(J,1,`flex items-center justify-center gap-1.5 py-2 rounded-xl text-[13px] font-semibold border transition-colors ${l.sidebarTab==="player"?"bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]":"border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]"}`,"svelte-1jr4poh"),re(G,1,`flex items-center justify-center gap-1.5 py-2 rounded-xl text-[13px] font-semibold border transition-colors ${l.sidebarTab==="lyrics"?"bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]":"border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]"}`,"svelte-1jr4poh"),re(oe,1,`flex items-center justify-center gap-1.5 py-2 rounded-xl text-[13px] font-semibold border transition-colors ${l.sidebarTab==="queue"?"bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]":"border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]"}`,"svelte-1jr4poh")}),m("click",Z,()=>{l.sidebarOpen=!1,l.persistSession()}),m("click",J,()=>l.sidebarTab="player"),m("click",G,()=>l.sidebarTab="lyrics"),m("click",oe,()=>l.sidebarTab="queue"),_(t,W),ht()}kt(["click","input","pointerdown","change","keydown"]);var qu=w('<span class="live-pill svelte-ja6svg">● Live</span>'),Qa=w('<div class="qr-box svelte-ja6svg"></div>'),zu=w('<div class="spin-card qr-section svelte-ja6svg"><span class="qr-kicker svelte-ja6svg">Scan to join</span> <!> <div class="copy-row svelte-ja6svg"><code class="svelte-ja6svg"> </code> <button class="pill-btn">Copy</button></div></div>'),Lu=w('<div class="spin-card svelte-ja6svg"><label class="switch-row svelte-ja6svg"><span class="svelte-ja6svg"><strong>Enable local Jukebox</strong> <small class="muted svelte-ja6svg">Local web server on your Wi-Fi</small></span> <input type="checkbox"/></label> <label class="switch-row svelte-ja6svg"><span class="svelte-ja6svg"><strong>Allow controls</strong> <small class="muted svelte-ja6svg">Pause, skip, and reorder queue</small></span> <input type="checkbox"/></label> <div class="status-line svelte-ja6svg"><span class="muted">Status</span> <span> </span></div></div> <!>',1),Ru=w('<div class="spin-card qr-section svelte-ja6svg"><span class="qr-kicker svelte-ja6svg">Active invitation</span> <!> <div class="copy-row svelte-ja6svg"><code class="svelte-ja6svg"> </code> <button class="pill-btn">Copy</button></div></div>'),Cu=w('<div class="spin-card svelte-ja6svg"><label class="switch-row svelte-ja6svg"><span class="svelte-ja6svg"><strong>Broadcast as host</strong> <small class="muted svelte-ja6svg">Create a synced listening invitation</small></span> <input type="checkbox"/></label></div> <!> <div class="spin-card join-card svelte-ja6svg"><strong>Join a room</strong> <p class="muted svelte-ja6svg">Paste a SPIN invitation to listen in sync.</p> <input class="text-input svelte-ja6svg" type="text" placeholder="GYRO-… or host URL"/> <button> </button></div>',1),Ou=w('<aside class="spin-sidebar w-[340px] shrink-0 h-full flex flex-col border-l border-[var(--border)] bg-[var(--bg-sidebar)] overflow-hidden max-xl:fixed max-xl:right-0 max-xl:top-14 max-xl:bottom-0 max-xl:z-40 max-xl:shadow-2xl svelte-ja6svg"><div class="spin-header svelte-ja6svg"><div class="spin-title-row svelte-ja6svg"><h3 class="svelte-ja6svg">SPIN Mode</h3> <!></div> <button class="icon-btn" title="Close SPIN panel">✕</button></div> <div class="spin-body scroll-y svelte-ja6svg"><div class="mode-switch svelte-ja6svg"><button>Wi-Fi Jukebox</button> <button>App to App</button></div> <!></div></aside>');function Fu(t,e){ft(e,!0);let r=q("local"),s=q(""),n=q(!1),i=me(()=>l.spinStatus.local_active||l.spinStatus.remote_role!=="idle");async function o(j){if(Le()){g(n,!0);try{l.spinStatus=j?await Ae.spinStartLocal():await Ae.spinStopLocal()}catch(ie){l.showToast(String(ie))}finally{g(n,!1)}}}async function c(j){if(Le())try{l.spinStatus=await Ae.spinSetAllowControls(j)}catch(ie){l.showToast(String(ie))}}async function v(j){if(Le()){g(n,!0);try{l.spinStatus=j?await Ae.spinStartRemoteHost():await Ae.spinStopRemote()}catch(ie){l.showToast(String(ie))}finally{g(n,!1)}}}async function d(){if(Le()){g(n,!0);try{l.spinStatus.remote_role==="guest"?l.spinStatus=await Ae.spinStopRemote():(l.spinStatus=await Ae.spinJoinRemote(a(s).trim()),l.showToast("Connecting to SPIN room…"))}catch(j){l.showToast(String(j))}finally{g(n,!1)}}}async function h(j,ie){try{await navigator.clipboard.writeText(j),l.showToast(ie)}catch{l.showToast("Could not copy to clipboard")}}var p=Ou(),b=f(p),y=f(b),x=u(f(y),2);{var A=j=>{var ie=qu();_(j,ie)};z(x,j=>{a(i)&&j(A)})}var k=u(y,2),P=u(b,2),E=f(P),W=f(E);let K;var he=u(W,2);let I;var M=u(E,2);{var Z=j=>{var ie=Lu(),N=ke(ie),ae=f(N),T=u(f(ae),2),B=u(ae,2),J=u(f(B),2),H=u(B,2),G=u(f(H),2),le=$(G,!0),oe=u(N,2);{var ye=F=>{var ce=zu(),D=u(f(ce),2);{var te=we=>{var Q=Qa();wa(Q,()=>l.spinStatus.local_qr_svg,!0),_(we,Q)};z(D,we=>{l.spinStatus.local_qr_svg&&we(te)})}var se=u(D,2),pe=f(se),ee=$(pe,!0),Ie=u(pe,2);L(()=>S(ee,l.spinStatus.local_url)),m("click",Ie,()=>void h(l.spinStatus.local_url,"SPIN link copied")),_(F,ce)};z(oe,F=>{l.spinStatus.local_active&&F(ye)})}L(()=>{T.disabled=a(n),Ur(T,l.spinStatus.local_active),Ur(J,l.spinStatus.allow_controls),S(le,l.spinStatus.local_active?`${l.spinStatus.guest_count} guests • ${l.spinStatus.added_count} added`:"Inactive")}),m("change",T,F=>void o(F.currentTarget.checked)),m("change",J,F=>void c(F.currentTarget.checked)),_(j,ie)},ue=j=>{var ie=Cu(),N=ke(ie),ae=f(N),T=u(f(ae),2),B=u(N,2);{var J=F=>{var ce=Ru(),D=u(f(ce),2);{var te=we=>{var Q=Qa();wa(Q,()=>l.spinStatus.remote_qr_svg,!0),_(we,Q)};z(D,we=>{l.spinStatus.remote_qr_svg&&we(te)})}var se=u(D,2),pe=f(se),ee=$(pe,!0),Ie=u(pe,2);L(()=>S(ee,l.spinStatus.remote_ticket)),m("click",Ie,()=>void h(l.spinStatus.remote_ticket??"","Invitation copied")),_(F,ce)};z(B,F=>{l.spinStatus.remote_role==="host"&&l.spinStatus.remote_ticket&&F(J)})}var H=u(B,2),G=u(f(H),4),le=u(G,2);let oe;var ye=$(le,!0);L(()=>{T.disabled=a(n),Ur(T,l.spinStatus.remote_role==="host"),oe=re(le,1,"pill-btn svelte-ja6svg",null,oe,{primary:l.spinStatus.remote_role!=="guest"}),le.disabled=a(n),S(ye,l.spinStatus.remote_role==="guest"?"Disconnect":"Join")}),m("change",T,F=>void v(F.currentTarget.checked)),vs(G,()=>a(s),F=>g(s,F)),m("click",le,()=>void d()),_(j,ie)};z(M,j=>{a(r)==="local"?j(Z):j(ue,-1)})}L(()=>{K=re(W,1,"svelte-ja6svg",null,K,{active:a(r)==="local"}),I=re(he,1,"svelte-ja6svg",null,I,{active:a(r)==="remote"})}),m("click",k,()=>l.spinOpen=!1),m("click",W,()=>g(r,"local")),m("click",he,()=>g(r,"remote")),_(t,p),ht()}kt(["click","change"]);var Du=w('<div class="fixed bottom-5 left-1/2 -translate-x-1/2 z-40 w-[min(560px,calc(100vw-32px))] flex items-center gap-2.5 px-3.5 py-2 rounded-full bg-[var(--bg-elevated)] border border-[var(--border-strong)] shadow-2xl"><button class="flex-1 min-w-0 flex items-center gap-2.5 text-left"><!> <div class="track-meta"><span class="track-title"> </span> <span class="track-sub"> </span></div></button> <button title="Favorite"><!></button> <button class="icon-btn" title="Previous"><!></button> <button class="play-circle !w-10 !h-10" title="Play / Pause"><!></button> <button class="icon-btn" title="Next"><!></button></div>'),Vu=w('<div class="px-4 py-2 rounded-full text-[13px] font-medium bg-[var(--bg-elevated)] border border-[var(--border-strong)] shadow-xl"> </div>'),Hu=w('<div class="fixed bottom-5 left-1/2 -translate-x-1/2 z-50 flex flex-col items-center gap-2 pointer-events-none" aria-live="polite"></div>'),Qu=w('<div class="h-screen w-screen overflow-hidden flex flex-col bg-[var(--bg)] text-[var(--text)]"><!> <div class="flex-1 min-h-0 flex overflow-hidden relative"><!> <main class="flex-1 min-w-0 h-full overflow-y-auto ambient-surface scroll-y"><nav class="flex items-center justify-center gap-2 pt-3.5 pb-1 px-4"><button><!> Home</button> <button><!> Explore</button> <button><!> Collection</button></nav> <!></main> <!></div> <!> <!> <!> <!></div>');function Uu(t,e){ft(e,!0),so(()=>{l.bootstrap()}),Ut(()=>{const T=l.ambientRgb;document.documentElement.style.setProperty("--ambient-rgb",T?`${T[0]}, ${T[1]}, ${T[2]}`:"33, 144, 164")});var r=Qu(),s=f(r);Cc(s,{});var n=u(s,2),i=f(n);{var o=T=>{ju(T,{})};z(i,T=>{l.sidebarOpen&&T(o)})}var c=u(i,2),v=f(c),d=f(v),h=f(d);fo(h,{size:15});var p=u(d,2),b=f(p);uo(b,{size:15});var y=u(p,2),x=f(y);ho(x,{size:15});var A=u(v,2);{var k=T=>{Yc(T,{})},P=T=>{Nc(T,{})},E=T=>{fc(T,{})},W=T=>{Jo(T,{get artistId(){return l.activeView.artistId},get artistName(){return l.activeView.artistName}})},K=T=>{Oo(T,{get id(){return l.activeView.id},get title(){return l.activeView.title},get coverUrl(){return l.activeView.coverUrl},get isPlaylist(){return l.activeView.isPlaylist}})};z(A,T=>{l.activeView.kind==="home"?T(k):l.activeView.kind==="explore"?T(P,1):l.activeView.kind==="collection"?T(E,2):l.activeView.kind==="artist"?T(W,3):l.activeView.kind==="album"&&T(K,4)})}var he=u(c,2);{var I=T=>{Fu(T,{})};z(he,T=>{l.spinOpen&&T(I)})}var M=u(n,2);{var Z=T=>{var B=Du(),J=f(B),H=f(J);Ge(H,{class:"w-10 h-10 rounded-lg object-cover shrink-0",get src(){return l.currentTrack.cover_url},alt:""});var G=u(H,2),le=f(G),oe=$(le,!0),ye=u(le,2),F=$(ye,!0),ce=u(J,2);let D;var te=f(ce);{let O=me(()=>l.isFavorite(l.currentTrack.id)?"currentColor":"none");Xn(te,{size:17,get fill(){return a(O)}})}var se=u(ce,2),pe=f(se);ti(pe,{size:17,fill:"currentColor"});var ee=u(se,2),Ie=f(ee);{var we=O=>{Zn(O,{size:17,fill:"currentColor"})},Q=O=>{ei(O,{size:17,fill:"currentColor",class:"translate-x-[1px]"})};z(Ie,O=>{l.isPlaying?O(we):O(Q,-1)})}var fe=u(ee,2),C=f(fe);ri(C,{size:17,fill:"currentColor"}),L(O=>{S(oe,l.currentTrack.title),S(F,l.currentTrack.artist),D=re(ce,1,"icon-btn",null,D,{"fav-active":O})},[()=>l.isFavorite(l.currentTrack.id)]),m("click",J,()=>l.sidebarOpen=!0),m("click",ce,()=>l.currentTrack&&l.toggleFavorite(l.currentTrack)),m("click",se,()=>void l.prev()),m("click",ee,()=>void l.togglePlay()),m("click",fe,()=>void l.next()),_(T,B)};z(M,T=>{!l.sidebarOpen&&l.currentTrack&&T(Z)})}var ue=u(M,2);{var j=T=>{ru(T,{})};z(ue,T=>{l.partyMode&&T(j)})}var ie=u(ue,2);au(ie,{});var N=u(ie,2);{var ae=T=>{var B=Hu();Oe(B,21,()=>l.toasts,J=>J.id,(J,H)=>{var G=Vu(),le=$(G,!0);L(()=>S(le,a(H).message)),_(J,G)}),_(T,B)};z(N,T=>{l.toasts.length>0&&T(ae)})}L(()=>{re(d,1,`inline-flex items-center gap-1.5 px-4 py-1.5 rounded-full text-[13px] font-semibold transition-colors ${l.activeTab==="home"?"bg-[var(--surface-hover)] text-[var(--text)] shadow-sm":"text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]"}`),re(p,1,`inline-flex items-center gap-1.5 px-4 py-1.5 rounded-full text-[13px] font-semibold transition-colors ${l.activeTab==="explore"?"bg-[var(--surface-hover)] text-[var(--text)] shadow-sm":"text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]"}`),re(y,1,`inline-flex items-center gap-1.5 px-4 py-1.5 rounded-full text-[13px] font-semibold transition-colors ${l.activeTab==="collection"?"bg-[var(--surface-hover)] text-[var(--text)] shadow-sm":"text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]"}`)}),m("click",d,()=>l.navigateTab("home")),m("click",p,()=>l.navigateTab("explore")),m("click",y,()=>l.navigateTab("collection")),_(t,r),ht()}kt(["click"]);Cl(Uu,{target:document.getElementById("app")});
