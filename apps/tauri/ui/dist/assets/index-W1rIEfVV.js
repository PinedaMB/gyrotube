(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const i of document.querySelectorAll('link[rel="modulepreload"]'))a(i);new MutationObserver(i=>{for(const o of i)if(o.type==="childList")for(const s of o.addedNodes)s.tagName==="LINK"&&s.rel==="modulepreload"&&a(s)}).observe(document,{childList:!0,subtree:!0});function r(i){const o={};return i.integrity&&(o.integrity=i.integrity),i.referrerPolicy&&(o.referrerPolicy=i.referrerPolicy),i.crossOrigin==="use-credentials"?o.credentials="include":i.crossOrigin==="anonymous"?o.credentials="omit":o.credentials="same-origin",o}function a(i){if(i.ep)return;i.ep=!0;const o=r(i);fetch(i.href,o)}})();const bo=!1;var Fa=Array.isArray,xo=Array.prototype.indexOf,pa=Array.prototype.includes,Ha=Array.from,Oi=Object.defineProperty,_r=Object.getOwnPropertyDescriptor,Di=Object.getOwnPropertyDescriptors,mo=Object.prototype,yo=Array.prototype,zn=Object.getPrototypeOf,Xn=Object.isExtensible;function jr(t){return typeof t=="function"}const qr=()=>{};function wo(t){return t()}function fn(t){for(var e=0;e<t.length;e++)t[e]()}function Fi(){var t,e,r=new Promise((a,i)=>{t=a,e=i});return{promise:r,resolve:t,reject:e}}function ko(t,e){if(Array.isArray(t))return t;if(!(Symbol.iterator in t))return Array.from(t);const r=[];for(const a of t)if(r.push(a),r.length===e)break;return r}const St=2,Hr=4,ma=8,Nn=1<<24,qt=16,Yt=32,br=64,hn=128,Mn=256,Gt=512,pt=1024,at=2048,Lt=4096,Nt=8192,wt=16384,Lr=32768,_a=1<<25,lr=65536,Ea=1<<17,So=1<<18,Gr=1<<19,Hi=1<<20,tr=1<<25,Ca=1<<21,Dr=1<<22,gr=1<<23,rr=Symbol("$state"),Vi=Symbol("component"),Qi=Symbol("legacy props"),$o=Symbol(""),Ui=Symbol("attributes"),pn=Symbol("class"),_n=Symbol("style"),gn=Symbol("text"),Na=Symbol("form reset"),ya=new class extends Error{name="StaleReactionError";message="The reaction that called `getAbortSignal()` was re-run or destroyed"},Va=!!globalThis.document?.contentType&&globalThis.document.contentType.includes("xml"),Po=1,To=2,Bi=4,Ao=8,zo=16,No=1,Mo=2,Wi=4,Eo=8,Co=16,Io=1,Lo=2,Ro=4,jo=1,qo=2,Gi={},ft=Symbol("uninitialized"),Yi="http://www.w3.org/1999/xhtml",Ki="http://www.w3.org/2000/svg",Oo="http://www.w3.org/1998/Math/MathML",Do="@attach";function Fo(){console.warn("https://svelte.dev/e/derived_inert")}function Ho(){console.warn("https://svelte.dev/e/select_multiple_invalid_value")}function Vo(){console.warn("https://svelte.dev/e/svelte_boundary_reset_noop")}let Ji=!1;function Xi(t){return t===this.v}function Qo(t,e){return t!=t?e==e:t!==e||t!==null&&typeof t=="object"||typeof t=="function"}function Zi(t){return!Qo(t,this.v)}function Uo(t){throw new Error("https://svelte.dev/e/lifecycle_outside_component")}function Bo(){throw new Error("https://svelte.dev/e/async_derived_orphan")}function Wo(t,e,r){throw new Error("https://svelte.dev/e/each_key_duplicate")}function Go(t){throw new Error("https://svelte.dev/e/effect_in_teardown")}function Yo(){throw new Error("https://svelte.dev/e/effect_in_unowned_derived")}function Ko(t){throw new Error("https://svelte.dev/e/effect_orphan")}function Jo(){throw new Error("https://svelte.dev/e/effect_update_depth_exceeded")}function Xo(t){throw new Error("https://svelte.dev/e/props_invalid_value")}function Zo(){throw new Error("https://svelte.dev/e/state_descriptors_fixed")}function el(){throw new Error("https://svelte.dev/e/state_prototype_fixed")}function tl(){throw new Error("https://svelte.dev/e/state_unsafe_mutation")}function rl(){throw new Error("https://svelte.dev/e/svelte_boundary_reset_onerror")}let Yr=!1,al=!1;function nl(){Yr=!0}let it=null;function Vr(t){it=t}function Ae(t,e=!1,r){it={p:it,i:!1,c:null,e:null,s:t,x:null,r:je,l:Yr&&!e?{s:null,u:null,$:[]}:null}}function ze(t){var e=it,r=e.e;if(r!==null){e.e=null;for(var a of r)gs(a)}return e.i=!0,it=e.p,En(t)}function En(t={}){return Oi(t,Vi,{value:!0}),t}function Kr(){return!Yr||it!==null&&it.l===null}let Ar=[];function es(){var t=Ar;Ar=[],fn(t)}function Ot(t){if(Ar.length===0&&!va){var e=Ar;queueMicrotask(()=>{e===Ar&&es()})}Ar.push(t)}function il(){for(;Ar.length>0;)es()}const sl=-7169;function ut(t,e){t.f=t.f&sl|e}function Cn(t){(t.f&Gt)!==0||t.deps===null?ut(t,pt):ut(t,Lt)}function ts(t,e){(t.f&at)!==0?e.set(t,at):(t.f&Lt)!==0&&e.get(t)!==at&&e.set(t,Lt),ut(t,pt)}function ol(t,e){if(e){const r=document.body;t.autofocus=!0,Ot(()=>{document.activeElement===r&&t.focus()})}}let Zn=!1;function ll(){Zn||(Zn=!0,document.addEventListener("reset",t=>{Promise.resolve().then(()=>{if(!t.defaultPrevented)for(const e of t.target.elements)e[Na]?.()})},{capture:!0}))}function kr(t){var e=He,r=je;Dt(null),Ft(null);try{return t()}finally{Dt(e),Ft(r)}}function cl(t,e,r,a=r){t.addEventListener(e,()=>kr(r));const i=t[Na];i?t[Na]=()=>{i(),a(!0)}:t[Na]=()=>a(!0),ll()}function rs(t,e,r,a){const i=Kr()?Qr:In;var o=t.filter(_=>!_.settled),s=e.map(i);if(r.length===0&&o.length===0){a(s);return}var l=je,v=dl(),f=o.length===1?o[0].promise:o.length>1?Promise.all(o.map(_=>_.promise)):null;function x(_){if((l.f&wt)===0){v();try{a([...s,..._])}catch(S){er(S,l)}Ia()}}var g=as();if(r.length===0){f.then(()=>x([])).finally(g);return}function k(){Promise.all(r.map(_=>ul(_))).then(x).catch(_=>er(_,l)).finally(g)}f?f.then(()=>{if((l.f&wt)!==0){g();return}v(),k(),Ia()}):k()}function dl(){var t=je,e=He,r=it,a=Le;return function(o=!0){Ft(t),Dt(e),Vr(r),o&&(t.f&wt)===0&&(a?.activate(),a?.apply())}}function Ia(t=!0){Ft(null),Dt(null),Vr(null),t&&Le?.deactivate()}function as(){var t=je,e=t.b,r=Le,a=!!e?.is_rendered();return e?.update_pending_count(1,r),r.increment(a,t),()=>{e?.update_pending_count(-1,r),r.decrement(a,t)}}function Qr(t){var e=St|at;return je!==null&&(je.f|=Gr),{ctx:it,deps:null,effects:null,equals:Xi,f:e,fn:t,reactions:null,rv:0,v:ft,wv:0,parent:je,ac:null}}const ca=Symbol("obsolete");function ul(t,e,r){let a=je;a===null&&Bo();var i=void 0,o=cr(ft),s=!He,l=new Set;return $l(()=>{var v=je,f=Fi();i=f.promise;try{Promise.resolve(t()).then(f.resolve,_=>{_!==ya&&f.reject(_)}).finally(Ia)}catch(_){f.reject(_),Ia()}var x=Le;if(s){if((v.f&Lr)!==0)var g=as();if(a.b?.is_rendered())x.async_deriveds.get(v)?.reject(ca);else for(const _ of l.values())_.reject(ca);l.add(f),x.async_deriveds.set(v,f)}const k=(_,S=void 0)=>{g?.(),l.delete(f),S!==ca&&(x.activate(),S?(o.f|=gr,Ur(o,S)):((o.f&gr)!==0&&(o.f^=gr),Ur(o,_)),x.deactivate())};f.promise.then(k,_=>k(null,_||"unknown"))}),Ua(()=>{for(const v of l)v.reject(ca)}),new Promise(v=>{function f(x){function g(){x===i?v(o):f(i)}x.then(g,g)}f(i)})}function J(t){const e=Qr(t);return Ss(e),e}function In(t){const e=Qr(t);return e.equals=Zi,e}function vl(t){var e=t.effects;if(e!==null){t.effects=null;for(var r=0;r<e.length;r+=1)kt(e[r])}}function Ln(t){var e,r=je,a=t.parent;if(!dr&&a!==null&&t.v!==ft&&(a.f&(wt|Nt))!==0)return Fo(),t.v;Ft(a);try{vl(t),e=As(t)}finally{Ft(r)}return e}function ns(t){var e=Ln(t);if(!t.equals(e)&&(t.wv=Ps(),(!Le?.is_fork||t.deps===null)&&(Le!==null?(Le.capture(t,e,!0),Fr?.capture(t,e,!0)):t.v=e,t.deps===null))){ut(t,pt);return}dr||(Ut!==null?(qn()||Le?.is_fork)&&Ut.set(t,e):Cn(t))}function fl(t){if(t.effects!==null)for(const e of t.effects)(e.teardown||e.ac)&&(e.teardown?.(),e.ac!==null&&kr(()=>{e.ac.abort(ya),e.ac=null}),e.fn!==null&&(e.teardown=qr),ba(e,0),Dn(e))}function is(t){if(t.effects!==null)for(const e of t.effects)e.teardown&&e.fn!==null&&Br(e)}let Za=null,Rr=null,Le=null,Fr=null,Ut=null,bn=null,va=!1,en=!1,fa=null,Ma=null;var ei=0;let hl=1;class xr{id=hl++;started=!1;linked=!0;prev=null;next=null;async_deriveds=new Map;current=new Map;previous=new Map;#r=new Set;#l=new Set;#e=0;#s=new Map;#a=null;#t=[];#i=[];#n=new Map;#o=new Map;#f=new Set;is_fork=!1;#c=!1;constructor(){Rr===null?Za=Rr=this:(Rr.next=this,this.prev=Rr),Rr=this}#h(){if(this.is_fork)return!0;for(const e of this.#s.keys())if(!this.#g(e))return!0;return!1}#g(e){for(var r=e;r!==null;r=r.parent)if(this.#o.has(r))return!0;return!1}skip_effect(e){this.#o.has(e)||this.#o.set(e,{d:[],m:[]}),this.#f.delete(e)}unskip_effect(e,r=a=>this.schedule(a)){var a=this.#o.get(e);if(a){this.#o.delete(e);for(var i of a.d)ut(i,at),r(i);for(i of a.m)ut(i,Lt),r(i)}this.#f.add(e)}#d(){var e=[];for(const o of this.#t)if(!((o.f&wt)!==0||(o.f&(at|Lt))===0)){for(var r=o,a=!1;r.parent!==null;){r=r.parent;var i=r.f;if((i&(br|Yt))!==0){if((i&pt)===0){a=!0;break}r.f^=pt}}a||e.push(r)}return this.#t=[],e}#p(){this.started=!0;for(const[l,v]of this.#n)ut(l,v),this.schedule(l);this.apply();for(var e=fa=[],r=[],a=Ma=[];this.#t.length>0;){ei++>1e3&&(this.#v(),_l());for(const l of this.#d())try{this.#x(l,e,r)}catch(v){throw ls(l),this.#h()||this.discard(),v}if(this.#t.length>0){for(const l of r)this.#t.push(l);for(const l of e)this.#t.push(l);r.length=0,e.length=0}}if(Le=null,a.length>0){var i=xr.ensure();for(const l of a)i.schedule(l)}if(fa=null,Ma=null,this.#h()){this.#_(r),this.#_(e);for(const[l,v]of this.#o)os(l,v);a.length>0&&Le.#p();return}const o=this.#w();if(o){this.#_(r),this.#_(e),o.#u(this);return}this.#n.clear();for(const l of this.#r)l(this);this.#r.clear(),Fr=this,ti(r),ti(e),Fr=null,this.#a?.resolve();var s=Le;this.#e===0&&(this.#t.length===0||s!==null)&&this.#v(),s!==null&&(ar.clear(),s.#p())}#x(e,r,a){e.f^=pt;for(var i=e.first;i!==null;){var o=i.f,s=(o&(Yt|br))!==0,l=s&&(o&pt)!==0,v=l||(o&Nt)!==0||this.#o.has(i);if(!v&&i.fn!==null){s?i.f^=pt:(o&Hr)!==0?r.push(i):ka(i)&&((o&qt)!==0&&this.add_dirty_reaction(i,Lt),Br(i));var f=i.first;if(f!==null){i=f;continue}}for(;i!==null;){var x=i.next;if(x!==null){i=x;break}i=i.parent}}}#w(){for(var e=this.prev;e!==null;e=e.prev)if(!e.is_fork&&this.#b(e))return e;return null}#b(e){for(const[r,[,a]]of this.current)if(!a&&e.current.has(r))return!0;return!1}#u(e){for(const[a,i]of e.current)!this.previous.has(a)&&e.previous.has(a)&&this.previous.set(a,e.previous.get(a)),this.current.set(a,i);for(const[a,i]of e.async_deriveds){const o=this.async_deriveds.get(a);o&&i.promise.then(o.resolve).catch(o.reject)}e.async_deriveds.clear(),this.transfer_reactions(e.#n);const r=a=>{var i=a.reactions;if(i!==null&&!((a.f&St)!==0&&(a.f&(at|Lt))===0))for(const l of i){var o=l.f;if((o&St)!==0)r(l);else{var s=l;o&(Dr|qt)&&!this.async_deriveds.has(s)&&(this.#n.delete(s),ut(s,at),this.schedule(s))}}};for(const a of this.current.keys())r(a);this.oncommit(()=>e.discard()),e.#v(),Le=this,this.#p()}#_(e){for(var r=0;r<e.length;r+=1)ts(e[r],this.#n)}capture(e,r,a=!1){e.v!==ft&&!this.previous.has(e)&&this.previous.set(e,e.v),(e.f&gr)===0&&(this.current.set(e,[r,a]),Ut?.set(e,r)),this.is_fork||(e.v=r)}activate(){Le=this}deactivate(){Le=null,Ut=null}flush(){try{en=!0,Le=this,this.#p()}finally{ei=0,bn=null,fa=null,Ma=null,en=!1,Le=null,Ut=null,ar.clear()}}discard(){for(const e of this.#l)e(this);this.#l.clear();for(const e of this.async_deriveds.values())e.reject(ca);this.#v(),this.#a?.resolve()}register_created_effect(e){this.#i.push(e)}#m(){for(let g=Za;g!==null;g=g.next){var e=g.id<this.id,r=[];for(const[k,[_,S]]of this.current){if(g.current.has(k)){var a=g.current.get(k)[0];if(e&&_!==a)g.current.set(k,[_,S]);else continue}r.push(k)}if(e)for(const[k,_]of this.async_deriveds){const S=g.async_deriveds.get(k);S&&_.promise.then(S.resolve).catch(S.reject)}var i=[...g.current.keys()].filter(k=>!g.current.get(k)[1]);if(!(!g.started||i.length===0)){var o=i.filter(k=>!this.current.has(k));if(o.length===0)e&&g.discard();else if(r.length>0){if(e)for(const k of this.#f)g.unskip_effect(k,_=>{(_.f&(qt|Dr))!==0?g.schedule(_):g.#_([_])});g.activate();var s=new Set,l=new Map;for(var v of r)ss(v,o,s,l);l=new Map;var f=[...g.current].filter(([k,_])=>{const S=this.current.get(k);return S?S[0]!==_[0]||S[1]!==_[1]:!0}).map(([k])=>k);if(f.length>0)for(const k of this.#i)(k.f&(wt|Nt|Ea))===0&&Rn(k,f,l)&&((k.f&(Dr|qt))!==0?(ut(k,at),g.schedule(k)):g.add_dirty_reaction(k,at));if(g.#t.length>0&&!g.#c){g.apply();for(var x of g.#d())g.#x(x,[],[])}g.deactivate()}}}}increment(e,r){if(this.#e+=1,e){let a=this.#s.get(r)??0;this.#s.set(r,a+1)}}decrement(e,r){if(this.#e-=1,e){let a=this.#s.get(r)??0;a===1?this.#s.delete(r):this.#s.set(r,a-1)}this.#c||(this.#c=!0,Ot(()=>{this.#c=!1,this.linked&&this.flush()}))}transfer_reactions(e){for(const[r,a]of e)this.add_dirty_reaction(r,a);e.clear()}add_dirty_reaction(e,r){(r===at||this.#n.get(e)!==at)&&this.#n.set(e,r)}oncommit(e){this.#r.add(e)}ondiscard(e){this.#l.add(e)}settled(){return(this.#a??=Fi()).promise}static ensure(){if(Le===null){const e=Le=new xr;!en&&!va&&Ot(()=>{e.started||e.flush()})}return Le}apply(){{Ut=null;return}}schedule(e){if(bn=e,e.b?.is_pending&&(e.f&(Hr|ma|Nn))!==0&&(e.f&Lr)===0){e.b.defer_effect(e);return}this.#t.push(e)}#v(){if(this.linked){var e=this.prev,r=this.next;e===null?Za=r:e.next=r,r===null?Rr=e:r.prev=e,this.linked=!1}}}function pl(t){var e=va,r=Fr;Fr=null,va=!0;try{for(var a;;){if(il(),Le===null)return a;Le.flush()}}finally{va=e,Fr=r}}function _l(){try{Jo()}catch(t){er(t,bn)}}let sr=null;function ti(t){var e=t.length;if(e!==0){for(var r=0;r<e;){var a=t[r++];if((a.f&(wt|Nt))===0&&ka(a)&&(sr=new Set,Br(a),a.deps===null&&a.first===null&&a.nodes===null&&a.teardown===null&&a.ac===null&&ys(a),sr?.size>0)){ar.clear();for(const i of sr){if((i.f&(wt|Nt))!==0)continue;const o=[i];let s=i.parent;for(;s!==null;)sr.has(s)&&(sr.delete(s),o.push(s)),s=s.parent;for(let l=o.length-1;l>=0;l--){const v=o[l];(v.f&(wt|Nt))===0&&Br(v)}}sr.clear()}}sr=null}}function ss(t,e,r,a){if(!r.has(t)&&(r.add(t),t.reactions!==null))for(const i of t.reactions){const o=i.f;(o&St)!==0?ss(i,e,r,a):(o&(Dr|qt))!==0&&(o&at)===0&&Rn(i,e,a)&&(ut(i,at),jn(i))}}function Rn(t,e,r){const a=r.get(t);if(a!==void 0)return a;if(t.deps!==null)for(const i of t.deps){if(pa.call(e,i))return!0;if((i.f&St)!==0&&Rn(i,e,r))return r.set(i,!0),!0}return r.set(t,!1),!1}function jn(t){Le.schedule(t)}function os(t,e){if(!((t.f&Yt)!==0&&(t.f&pt)!==0)){(t.f&at)!==0?e.d.push(t):(t.f&Lt)!==0&&e.m.push(t),ut(t,pt);for(var r=t.first;r!==null;)os(r,e),r=r.next}}function ls(t){ut(t,pt);for(var e=t.first;e!==null;)ls(e),e=e.next}let La=new Set;const ar=new Map;let cs=!1;function cr(t,e){var r={f:0,v:t,reactions:null,equals:Xi,rv:0,wv:0};return r}function G(t,e){const r=cr(t);return Ss(r),r}function gl(t,e=!1,r=!0){const a=cr(t);return e||(a.equals=Zi),Yr&&r&&it!==null&&it.l!==null&&(it.l.s??=[]).push(a),a}function z(t,e,r=!1){He!==null&&(!Wt||(He.f&Ea)!==0)&&Kr()&&(He.f&(St|qt|Dr|Ea))!==0&&(ir===null||!ir.has(t))&&tl();let a=r?Je(e):e;return Ur(t,a,Ma)}var Tr=null,xn=0;function Ur(t,e,r=null){if(!t.equals(e)){dr?ar.set(t,e):ar.has(t)||ar.set(t,t.v);var a=xr.ensure();if(a.capture(t,e),(t.f&St)!==0){const i=t;(t.f&at)!==0&&Ln(i),Ut===null&&Cn(i)}t.wv=Ps(),Tr=null,xn=0,ds(t,at,r),Tr=null,Kr()&&je!==null&&(je.f&pt)!==0&&(je.f&(Yt|br))===0&&(jt===null?Tl([t]):jt.push(t)),!a.is_fork&&La.size>0&&!cs&&bl()}return e}function bl(){cs=!1;for(const t of La){(t.f&pt)!==0&&ut(t,Lt);let e;try{e=ka(t)}catch{e=!0}e&&Br(t)}La.clear()}function ri(t,e=1){var r=n(t),a=e===1?r++:r--;return z(t,r),a}function ha(t){z(t,t.v+1)}function ds(t,e,r){var a=t.reactions;if(a!==null){var i=Kr(),o=a.length;if(xn+=o,xn>1e5&&Tr===null&&(Tr=new Set),Tr!==null){if(Tr.has(t))return;Tr.add(t)}for(var s=0;s<o;s++){var l=a[s],v=l.f;if(!(!i&&l===je)){var f=(v&at)===0;if(f&&ut(l,e),(v&Ea)!==0)La.add(l);else if((v&St)!==0){var x=l;Ut?.delete(x),ds(x,Lt,r)}else if(f){var g=l;(v&qt)!==0&&sr!==null&&sr.add(g),r!==null?r.push(g):jn(g)}}}}}function Je(t){if(typeof t!="object"||t===null||rr in t||Vi in t)return t;const e=zn(t);if(e!==mo&&e!==yo)return t;var r=new Map,a=Fa(t),i=G(0),o=Mr,s=l=>{if(Mr===o)return l();var v=He,f=Mr;Dt(null),ii(o);var x=l();return Dt(v),ii(f),x};return a&&r.set("length",G(t.length)),new Proxy(t,{defineProperty(l,v,f){(!("value"in f)||f.configurable===!1||f.enumerable===!1||f.writable===!1)&&Zo();var x=r.get(v);return x===void 0?s(()=>{var g=G(f.value);return r.set(v,g),g}):z(x,f.value,!0),!0},deleteProperty(l,v){var f=r.get(v);if(f===void 0){if(v in l){const x=s(()=>G(ft));r.set(v,x),ha(i)}}else z(f,ft),ha(i);return!0},get(l,v,f){if(v===rr)return t;var x=r.get(v),g=v in l;if(x===void 0&&(!g||_r(l,v)?.writable)&&(x=s(()=>{var _=Je(g?l[v]:ft),S=G(_);return S}),r.set(v,x)),x!==void 0){var k=n(x);return k===ft?void 0:k}return Reflect.get(l,v,f)},getOwnPropertyDescriptor(l,v){this.has?.(l,v);var f=Reflect.getOwnPropertyDescriptor(l,v),x=r.get(v);if(x!==void 0){var g=n(x);if(g===ft)return;if(f&&"value"in f)f.value=g;else return{enumerable:!0,configurable:!0,value:g,writable:!0}}return f},has(l,v){if(v===rr)return!0;var f=r.get(v),x=f!==void 0&&f.v!==ft||Reflect.has(l,v);if(f!==void 0||(je!==null||He!==null)&&(!x||_r(l,v)?.writable)){f===void 0&&(f=s(()=>{var k=x?Je(l[v]):ft,_=G(k);return _}),r.set(v,f));var g=n(f);if(g===ft)return!1}return x},set(l,v,f,x){var g=r.get(v),k=v in l;if(a&&v==="length")for(var _=f;_<g.v;_+=1){var S=r.get(_+"");S!==void 0?z(S,ft):_ in l&&(S=s(()=>G(ft)),r.set(_+"",S))}if(g===void 0)(!k||_r(l,v)?.writable)&&(g=s(()=>G(void 0)),z(g,Je(f)),r.set(v,g));else{k=g.v!==ft;var I=s(()=>Je(f));z(g,I)}var $=Reflect.getOwnPropertyDescriptor(l,v);if($?.set&&$.set.call(x,f),!k){if(a&&typeof v=="string"){var y=r.get("length"),b=Number(v);Number.isInteger(b)&&b>=y.v&&z(y,b+1)}ha(i)}return!0},ownKeys(l){n(i);var v=Reflect.ownKeys(l).filter(g=>{var k=r.get(g);return k===void 0||k.v!==ft});for(var[f,x]of r)x.v!==ft&&!(f in l)&&v.push(f);return v},setPrototypeOf(){el()}})}function ai(t){try{if(t!==null&&typeof t=="object"&&rr in t)return t[rr]}catch{}return t}function us(t,e){return Object.is(ai(t),ai(e))}var ga,vs,fs,hs;function xl(){if(ga===void 0){ga=window,vs=/Firefox/.test(navigator.userAgent);var t=Element.prototype,e=Node.prototype,r=Text.prototype;fs=_r(e,"firstChild").get,hs=_r(e,"nextSibling").get,Xn(t)&&(t[pn]=void 0,t[Ui]=null,t[_n]=void 0,t.__e=void 0),Xn(r)&&(r[gn]=void 0)}}function nr(t=""){return document.createTextNode(t)}function Bt(t){return fs.call(t)}function wa(t){return hs.call(t)}function u(t,e){return Bt(t)}function K(t,e=!1){{var r=Bt(t);return r instanceof Comment&&r.data===""?wa(r):r}}function T(t,e=!1){return Bt(t)}function d(t,e=1,r=!1){let a=t;for(;e--;)a=wa(a);return a}function ml(t){t.textContent=""}function ps(){return!1}function Qa(t,e,r){return e==null||e===Yi?r?document.createElement(t,{is:r}):document.createElement(t):r?document.createElementNS(e,t,{is:r}):document.createElementNS(e,t)}function yl(t){var e=je;if(e===null)return He.f|=gr,t;if((e.f&Lr)===0&&(e.f&Hr)===0)throw t;er(t,e)}function er(t,e){if(t===Gi)throw t;if(!(e!==null&&(e.f&wt)!==0)){for(;e!==null;){if((e.f&hn)!==0&&(e.f&(wt|_a))===0){if((e.f&Lr)===0)throw t;try{e.b.error(t);return}catch(r){t=r}}e=e.parent}throw t}}function _s(t){je===null&&(He===null&&Ko(),Yo()),dr&&Go()}function wl(t,e){var r=e.last;r===null?e.last=e.first=t:(r.next=t,t.prev=r,e.last=t)}function Kt(t,e){var r=je;r!==null&&(r.f&Nt)!==0&&(t|=Nt);var a={ctx:it,deps:null,nodes:null,f:t|at|Gt,first:null,fn:e,last:null,next:null,parent:r,b:r&&r.b,prev:null,teardown:null,wv:0,ac:null};Le?.register_created_effect(a);var i=a;if((t&Hr)!==0)fa!==null?fa.push(a):xr.ensure().schedule(a);else if(e!==null){try{Br(a)}catch(s){throw kt(a),s}i.deps===null&&i.teardown===null&&i.nodes===null&&i.first===i.last&&(i.f&Gr)===0&&(i=i.first,(t&qt)!==0&&(t&lr)!==0&&i!==null&&(i.f|=lr))}if(i!==null&&(i.parent=r,r!==null&&wl(i,r),He!==null&&(He.f&St)!==0&&(t&br)===0)){var o=He;(o.effects??=[]).push(i)}return a}function qn(){return He!==null&&!Wt}function Ua(t){const e=Kt(ma,null);return ut(e,pt),e.teardown=t,e}function $t(t){_s();var e=je.f,r=!He&&(e&Yt)!==0&&it!==null&&!it.i;if(r){var a=it;(a.e??=[]).push(t)}else return gs(t)}function gs(t){return Kt(Hr|Hi,t)}function kl(t){return _s(),Kt(ma|Hi,t)}function Sl(t){xr.ensure();const e=Kt(br|Gr,t);return(r={})=>new Promise(a=>{r.outro?Nr(e,()=>{kt(e),a(void 0)}):(kt(e),a(void 0))})}function Ba(t){return Kt(Hr,t)}function $l(t){return Kt(Dr|Gr,t)}function On(t,e=0){return Kt(ma|e,t)}function O(t,e=[],r=[],a=[]){rs(a,e,r,i=>{Kt(ma,()=>{t(...i.map(n))})})}function Jr(t,e=0){var r=Kt(qt|e,t);return r}function bs(t,e=0){var r=Kt(Nn|e,t);return r}function Et(t){return Kt(Yt|Gr,t)}function xs(t){var e=t.teardown;if(e!==null){const r=dr,a=He;ni(!0),Dt(null);try{e.call(null)}catch(i){er(i,t.parent)}finally{ni(r),Dt(a)}}}function Dn(t,e=!1){var r=t.first;for(t.first=t.last=null;r!==null;){const i=r.ac;i!==null&&kr(()=>{i.abort(ya)});var a=r.next;(r.f&br)!==0?r.parent=null:kt(r,e),r=a}}function Pl(t){for(var e=t.first;e!==null;){var r=e.next;(e.f&Yt)===0&&kt(e),e=r}}function kt(t,e=!0){var r=!1;(e||(t.f&So)!==0)&&t.nodes!==null&&t.nodes.end!==null&&(ms(t.nodes.start,t.nodes.end),r=!0),t.f|=_a,Dn(t,e&&!r),ba(t,0);var a=t.nodes&&t.nodes.t;if(a!==null)for(const o of a)o.stop();xs(t),t.f^=_a,t.f|=wt;var i=t.parent;i!==null&&i.first!==null&&ys(t),t.next=t.prev=t.teardown=t.ctx=t.deps=t.fn=t.nodes=t.ac=t.b=null}function ms(t,e){for(;t!==null;){var r=t===e?null:wa(t);t.remove(),t=r}}function ys(t){var e=t.parent,r=t.prev,a=t.next;r!==null&&(r.next=a),a!==null&&(a.prev=r),e!==null&&(e.first===t&&(e.first=a),e.last===t&&(e.last=r))}function Nr(t,e,r=!0){var a=[];t.f|=Mn,ws(t,a,!0);var i=()=>{r&&kt(t),e&&e()},o=a.length;if(o>0){var s=()=>--o||i();for(var l of a)l.out(s)}else i()}function ws(t,e,r){if((t.f&Nt)===0){t.f^=Nt;var a=t.nodes&&t.nodes.t;if(a!==null)for(const l of a)(l.is_global||r)&&e.push(l);for(var i=t.first;i!==null;){var o=i.next;if((i.f&br)===0){var s=(i.f&lr)!==0||(i.f&Yt)!==0&&(t.f&qt)!==0;ws(i,e,s?r:!1)}i=o}}}function Ra(t){t.f&=~Mn,ks(t,!0)}function ks(t,e){if((t.f&Mn)===0&&(t.f&Nt)!==0){t.f^=Nt,(t.f&pt)===0&&(ut(t,at),xr.ensure().schedule(t));for(var r=t.first;r!==null;){var a=r.next,i=(r.f&lr)!==0||(r.f&Yt)!==0;ks(r,i?e:!1),r=a}var o=t.nodes&&t.nodes.t;if(o!==null)for(const s of o)(s.is_global||e)&&s.in()}}function Fn(t,e){if(t.nodes)for(var r=t.nodes.start,a=t.nodes.end;r!==null;){var i=r===a?null:wa(r);e.append(r),r=i}}let dr=!1;function ni(t){dr=t}let He=null,Wt=!1;function Dt(t){He=t}let je=null;function Ft(t){je=t}let ir=null;function Ss(t){He!==null&&((He.f&Ca)!==0||(He.f&St)!==0)&&(ir??=new Set).add(t)}let Ct=null,It=0,jt=null;function Tl(t){jt=t}let $s=1,zr=0,Mr=zr;function ii(t){Mr=t}function Ps(){return++$s}function ka(t){var e=t.f;if((e&at)!==0)return!0;if((e&Lt)!==0){for(var r=t.deps,a=r.length,i=0;i<a;i++){var o=r[i];if(ka(o)&&ns(o),o.wv>t.wv)return!0}(e&Gt)!==0&&Ut===null&&ut(t,pt)}return!1}function Ts(t,e,r=!0){var a=t.reactions;if(a!==null&&!(ir!==null&&ir.has(t)))for(var i=0;i<a.length;i++){var o=a[i];(o.f&St)!==0?Ts(o,e,!1):e===o&&(r?ut(o,at):(o.f&pt)!==0&&ut(o,Lt),jn(o))}}function As(t){var e=Ct,r=It,a=jt,i=He,o=ir,s=it,l=Wt,v=Mr,f=t.f;Ct=null,It=0,jt=null,He=(f&(Yt|br))===0?t:null,ir=null,Vr(t.ctx),Wt=!1,Mr=++zr,t.ac!==null&&(kr(()=>{t.ac.abort(ya)}),t.ac=null);try{t.f|=Ca;var x=t.fn,g=x();t.f|=Lr;var k=si(t);if(Kr()&&jt!==null&&!Wt&&k!==null&&(t.f&(St|Lt|at))===0)for(var _=0;_<jt.length;_++)Ts(jt[_],t);if(i!==null&&i!==t){if(zr++,i.deps!==null)for(let S=0;S<r;S+=1)i.deps[S].rv=zr;if(e!==null)for(const S of e)S.rv=zr;jt!==null&&(a===null?a=jt:a.push(...jt))}return(t.f&gr)!==0&&(t.f^=gr),g}catch(S){return si(t),yl(S)}finally{t.f^=Ca,Ct=e,It=r,jt=a,He=i,ir=o,Vr(s),Wt=l,Mr=v}}function si(t){var e=t.deps,r=Le?.is_fork;if(Ct!==null){var a;if(r||ba(t,It),e!==null&&It>0)for(e.length=It+Ct.length,a=0;a<Ct.length;a++)e[It+a]=Ct[a];else t.deps=e=Ct;if(qn()&&(t.f&Gt)!==0)for(a=It;a<e.length;a++)(e[a].reactions??=[]).push(t)}else!r&&e!==null&&It<e.length&&(ba(t,It),e.length=It);return e}function Al(t,e){let r=e.reactions;if(r!==null){var a=xo.call(r,t);if(a!==-1){var i=r.length-1;i===0?r=e.reactions=null:(r[a]=r[i],r.pop())}}if(r===null&&(e.f&St)!==0&&(Ct===null||!pa.call(Ct,e))){var o=e;(o.f&Gt)!==0&&(o.f^=Gt),o.v!==ft&&Cn(o),o.ac!==null&&kr(()=>{o.ac.abort(ya),o.ac=null,ut(o,at)}),fl(o),ba(o,0)}}function ba(t,e){var r=t.deps;if(r!==null)for(var a=e;a<r.length;a++)Al(t,r[a])}function Br(t){var e=t.f;if((e&wt)===0){ut(t,pt);var r=je;je=t;try{(e&(qt|Nn))!==0?Pl(t):Dn(t),xs(t);var a=As(t);t.teardown=typeof a=="function"?a:null,t.wv=$s;var i;bo&&al&&(t.f&at)!==0&&t.deps}finally{je=r}}}async function zl(){await Promise.resolve(),pl()}function n(t){var e=t.f,r=(e&St)!==0;if(He!==null&&!Wt){var a=je!==null&&(je.f&wt)!==0;if(!a&&(ir===null||!ir.has(t))){var i=He.deps;if((He.f&Ca)!==0)t.rv<zr&&(t.rv=zr,Ct===null&&i!==null&&i[It]===t?It++:Ct===null?Ct=[t]:Ct.push(t));else{He.deps??=[],pa.call(He.deps,t)||He.deps.push(t);var o=t.reactions;o===null?t.reactions=[He]:pa.call(o,He)||o.push(He)}}}if(dr&&ar.has(t))return ar.get(t);if(r){var s=t;if(dr){var l=s.v;return((s.f&pt)===0&&s.reactions!==null||Ns(s))&&(l=Ln(s)),ar.set(s,l),l}var v=(s.f&Gt)===0&&!Wt&&He!==null&&(He.f&Gt)!==0,f=(s.f&Lr)===0;ka(s)&&(v&&(s.f|=Gt),ns(s)),v&&!f&&(is(s),zs(s))}if(Ut?.has(t))return Ut.get(t);if((t.f&gr)!==0)throw t.v;return t.v}function zs(t){if(t.f|=Gt,t.deps!==null)for(const r of t.deps){var e=r.reactions;e===null?r.reactions=[t]:pa.call(e,t)||e.push(t),(r.f&St)!==0&&(r.f&Gt)===0&&(is(r),zs(r))}}function Ns(t){if(t.v===ft)return!0;if(t.deps===null)return!1;for(const e of t.deps)if(ar.has(e)||(e.f&St)!==0&&Ns(e))return!0;return!1}function ur(t){var e=Wt;try{return Wt=!0,t()}finally{Wt=e}}function Pr(t){if(!(typeof t!="object"||!t||t instanceof EventTarget)){if(rr in t)mn(t);else if(!Array.isArray(t))for(let e in t){const r=t[e];typeof r=="object"&&r&&rr in r&&mn(r)}}}function mn(t,e=new Set){if(typeof t=="object"&&t!==null&&!(t instanceof EventTarget)&&!e.has(t)){e.add(t),t instanceof Date&&t.getTime();for(let a in t)try{mn(t[a],e)}catch{}const r=zn(t);if(r!==Object.prototype&&r!==Array.prototype&&r!==Map.prototype&&r!==Set.prototype&&r!==Date.prototype){const a=Di(r);for(let i in a){const o=a[i].get;if(o)try{o.call(t)}catch{}}}}}function Nl(t){return t.endsWith("capture")&&t!=="gotpointercapture"&&t!=="lostpointercapture"}const Ml=["beforeinput","click","change","dblclick","contextmenu","focusin","focusout","input","keydown","keyup","mousedown","mousemove","mouseout","mouseover","mouseup","pointerdown","pointermove","pointerout","pointerover","pointerup","touchend","touchmove","touchstart"];function El(t){return Ml.includes(t)}const Cl={formnovalidate:"formNoValidate",ismap:"isMap",nomodule:"noModule",playsinline:"playsInline",readonly:"readOnly",defaultvalue:"defaultValue",defaultchecked:"defaultChecked",srcobject:"srcObject",novalidate:"noValidate",allowfullscreen:"allowFullscreen",disablepictureinpicture:"disablePictureInPicture",disableremoteplayback:"disableRemotePlayback"};function Il(t){return t=t.toLowerCase(),Cl[t]??t}const Ll=["touchstart","touchmove"];function Rl(t){return Ll.includes(t)}const da=Symbol("events"),Ms=new Set,yn=new Set;function Es(t,e,r,a={}){function i(o){if(a.capture||wn.call(e,o),!o.cancelBubble)return kr(()=>r?.call(this,o))}return t.startsWith("pointer")||t.startsWith("touch")||t==="wheel"?(i.__removed=!1,Ot(()=>{i.__removed||e.addEventListener(t,i,a)})):e.addEventListener(t,i,a),i}function _t(t,e,r,a,i){var o={capture:a,passive:i},s=Es(t,e,r,o);(e===document.body||e===window||e===document||e instanceof HTMLMediaElement)&&Ua(()=>{s.__removed=!0,e.removeEventListener(t,s,o)})}function M(t,e,r){(e[da]??={})[t]=r}function Ve(t){for(var e=0;e<t.length;e++)Ms.add(t[e]);for(var r of yn)r(t)}let tn=null,rn=!1;function wn(t){var e=this,r=e.ownerDocument,a=t.type,i=t.composedPath?.()||[],o=i[0]||t.target;tn=t,rn||(rn=!0,setTimeout(()=>{rn=!1,tn=null}));var s=0,l=tn===t&&t[da];if(l){var v=i.indexOf(l);if(v!==-1&&(e===document||e===window)){t[da]=e;return}var f=i.indexOf(e);if(f===-1)return;v<=f&&(s=v)}if(o=i[s]||t.target,o!==e){Oi(t,"currentTarget",{configurable:!0,get(){return o||r}});var x=He,g=je;Dt(null),Ft(null);try{for(var k,_=[];o!==null&&o!==e;){try{var S=o[da]?.[a];S!=null&&(!o.disabled||t.target===o)&&S.call(o,t)}catch(I){k?_.push(I):k=I}if(t.cancelBubble)break;s++,o=s<i.length?i[s]:null}if(k){for(let I of _)queueMicrotask(()=>{throw I});throw k}}finally{t[da]=e,delete t.currentTarget,Dt(x),Ft(g)}}}const jl=globalThis?.window?.trustedTypes&&globalThis.window.trustedTypes.createPolicy("svelte-trusted-html",{createHTML:t=>t});function ql(t){return jl?.createHTML(t)??t}function Cs(t){var e=Qa("template");return e.innerHTML=ql(t.replaceAll("<!>","<!---->")),e.content}function mr(t,e){var r=je;r.nodes===null&&(r.nodes={start:t,end:e,a:null,t:null})}function L(t,e){var r=(e&jo)!==0,a=(e&qo)!==0,i,o=!t.startsWith("<!>");return()=>{i===void 0&&(i=Cs(o?t:"<!>"+t),r||(i=Bt(i)));var s=a||vs?document.importNode(i,!0):i.cloneNode(!0);if(r){var l=Bt(s),v=s.lastChild;mr(l,v)}else mr(s,s);return s}}function Ol(t,e,r="svg"){var a=!t.startsWith("<!>"),i=`<${r}>${a?t:"<!>"+t}</${r}>`,o;return()=>{if(!o){var s=Cs(i),l=Bt(s);o=Bt(l)}var v=o.cloneNode(!0);return mr(v,v),v}}function Dl(t,e){return Ol(t,e,"svg")}function Or(t=""){{var e=nr(t+"");return mr(e,e),e}}function se(){var t=document.createDocumentFragment(),e=document.createComment(""),r=nr();return t.append(e,r),mr(e,r),t}function h(t,e){t!==null&&t.before(e)}function Fl(t){let e=0,r=cr(0),a;return()=>{qn()&&(n(r),On(()=>(e===0&&(a=ur(()=>t(()=>ha(r)))),e+=1,()=>{Ot(()=>{e-=1,e===0&&(a?.(),a=void 0,ha(r))})})))}}var Hl=lr|Gr;function Vl(t,e,r,a){new Ql(t,e,r,a)}class Ql{parent;is_pending=!1;transform_error;#r;#l=null;#e;#s;#a;#t=null;#i=null;#n=null;#o=null;#f=0;#c=0;#h=!1;#g=new Map;#d=null;#p=Fl(()=>(this.#d=cr(this.#f),()=>{this.#d=null}));constructor(e,r,a,i){this.#r=e,this.#e=r,this.#s=o=>{var s=je;s.b=this,s.f|=hn,a(o)},this.parent=je.b,this.transform_error=i??this.parent?.transform_error??(o=>o),this.#a=Jr(()=>{this.#m()},Hl)}#x(){try{this.#t=Et(()=>this.#s(this.#r))}catch(e){this.error(e)}}#w(e){const r=this.#e.failed,{reset:a,invoke_onerror:i}=this.#b(e);Ot(i),r&&(this.#n=Et(()=>{r(this.#r,()=>e,()=>a)}))}#b(e){var r=!1,a=!1;const i=()=>{if(!this.#u()){if(r){Vo();return}r=!0,a&&rl(),this.#n!==null&&Nr(this.#n,()=>{this.#n=null}),this.#y(()=>{this.#m()})}};return{reset:i,invoke_onerror:()=>{if(!this.#u())try{a=!0,this.#e.onerror?.(e,i),a=!1}catch(s){er(s,this.#a&&this.#a.parent)}}}}#u(){return(this.#a.f&(wt|_a))!==0}#_(){const e=this.#e.pending;e&&(this.is_pending=!0,this.#i=Et(()=>e(this.#r)),Ot(()=>{if(!this.#u()){var r=this.#o=document.createDocumentFragment(),a=nr(),i=!1;if(r.append(a),this.#t=this.#y(()=>{try{return Et(()=>this.#s(a))}catch(o){try{this.error(o),i=!0}catch(s){er(s,this.#a.parent)}return null}}),this.#t===null){this.#o=null,i&&this.#v(Le);return}this.#c===0&&(this.#r.before(r),this.#o=null,Nr(this.#i,()=>{this.#i=null}),this.#v(Le))}}))}#m(){try{if(this.is_pending=this.has_pending_snippet(),this.#c=0,this.#f=0,this.#t=Et(()=>{this.#s(this.#r)}),this.#c>0){var e=this.#o=document.createDocumentFragment();Fn(this.#t,e);const r=this.#e.pending;this.#i=Et(()=>r(this.#r))}else this.#v(Le)}catch(r){this.error(r)}}#v(e){this.is_pending=!1,e.transfer_reactions(this.#g)}defer_effect(e){ts(e,this.#g)}is_rendered(){return!this.is_pending&&(!this.parent||this.parent.is_rendered())}has_pending_snippet(){return!!this.#e.pending}#y(e){var r=je,a=He,i=it;Ft(this.#a),Dt(this.#a),Vr(this.#a.ctx);try{return xr.ensure(),e()}finally{Ft(r),Dt(a),Vr(i)}}#k(e,r){if(!this.has_pending_snippet()){this.parent&&this.parent.#k(e,r);return}this.#c+=e,this.#c===0&&(this.#v(r),this.#i&&Nr(this.#i,()=>{this.#i=null}),this.#o&&(this.#r.before(this.#o),this.#o=null))}update_pending_count(e,r){this.#k(e,r),this.#f+=e,!(!this.#d||this.#h)&&(this.#h=!0,Ot(()=>{this.#h=!1,this.#d&&Ur(this.#d,this.#f)}))}get_effect_pending(){return this.#p(),n(this.#d)}error(e){if(e===Gi||!this.#e.onerror&&!this.#e.failed)throw e;Le?.is_fork?(this.#t&&Le.skip_effect(this.#t),this.#i&&Le.skip_effect(this.#i),this.#n&&Le.skip_effect(this.#n),Le.oncommit(()=>{this.#u()||this.#S(e)})):this.#S(e)}#S(e){this.#t&&(kt(this.#t),this.#t=null),this.#i&&(kt(this.#i),this.#i=null),this.#n&&(kt(this.#n),this.#n=null);let r=this.#e.failed;const a=i=>{if(this.#u())return;const{reset:o,invoke_onerror:s}=this.#b(i);s(),r&&!this.#u()&&(this.#n=this.#y(()=>{try{return Et(()=>{var l=je;l.b=this,l.f|=hn,r(this.#r,()=>i,()=>o)})}catch(l){return er(l,this.#a.parent),null}}))};Ot(()=>{if(!this.#u()){var i;try{i=this.transform_error(e)}catch(o){er(o,this.#a&&this.#a.parent);return}i!==null&&typeof i=="object"&&typeof i.then=="function"?i.then(a,o=>er(o,this.#a&&this.#a.parent)):a(i)}})}}let ja=!0;function Pa(t){ja=t}function m(t,e){var r=e==null?"":typeof e=="object"?`${e}`:e;r!==(t[gn]??=t.nodeValue)&&(t[gn]=r,t.nodeValue=`${r}`)}function Ul(t,e){return Bl(t,e)}const Ta=new Map;function Bl(t,{target:e,anchor:r,props:a={},events:i,context:o,intro:s=!0,transformError:l}){xl();var v=void 0,f=Sl(()=>{var x=r??e.appendChild(nr());Vl(x,{pending:()=>{}},_=>{Ae({});var S=it;o&&(S.c=o),i&&(a.$$events=i),ja=s,v=t(_,a)||En(),ja=!0,ze()},l);var g=new Set,k=_=>{for(var S=0;S<_.length;S++){var I=_[S];if(!g.has(I)){g.add(I);var $=Rl(I);for(const w of[e,document]){var y=Ta.get(w);y===void 0&&(y=new Map,Ta.set(w,y));var b=y.get(I);b===void 0?(w.addEventListener(I,wn,{passive:$}),y.set(I,1)):y.set(I,b+1)}}}};return k(Ha(Ms)),yn.add(k),()=>{for(var _ of g)for(const $ of[e,document]){var S=Ta.get($),I=S.get(_);--I==0?($.removeEventListener(_,wn),S.delete(_),S.size===0&&Ta.delete($)):S.set(_,I)}yn.delete(k),x!==r&&x.parentNode?.removeChild(x)}});return Wl.set(v,f),v}let Wl=new WeakMap;class Wa{anchor;#r=new Map;#l=new Map;#e=new Map;#s=new Set;#a=!0;constructor(e,r=!0){this.anchor=e,this.#a=r}#t=e=>{if(this.#r.has(e)){var r=this.#r.get(e),a=this.#l.get(r);if(a)Ra(a),this.#s.delete(r);else{var i=this.#e.get(r);i&&(Ra(i.effect),this.#l.set(r,i.effect),this.#e.delete(r),i.fragment.lastChild.remove(),this.anchor.before(i.fragment),a=i.effect)}for(const[o,s]of this.#r){if(this.#r.delete(o),o===e)break;const l=this.#e.get(s);l&&(kt(l.effect),this.#e.delete(s))}for(const[o,s]of this.#l){if(o===r||this.#s.has(o))continue;const l=()=>{if(Array.from(this.#r.values()).includes(o)){var f=document.createDocumentFragment();Fn(s,f),f.append(nr()),this.#e.set(o,{effect:s,fragment:f})}else kt(s);this.#s.delete(o),this.#l.delete(o)};this.#a||!a?(this.#s.add(o),Nr(s,l,!1)):l()}}};#i=e=>{this.#r.delete(e);const r=Array.from(this.#r.values());for(const[a,i]of this.#e)r.includes(a)||(kt(i.effect),this.#e.delete(a))};ensure(e,r){var a=Le,i=ps();if(r&&!this.#l.has(e)&&!this.#e.has(e))if(i){var o=document.createDocumentFragment(),s=nr();o.append(s),this.#e.set(e,{effect:Et(()=>r(s)),fragment:o})}else this.#l.set(e,Et(()=>r(this.anchor)));if(this.#r.set(a,e),i){for(const[l,v]of this.#l)l===e?a.unskip_effect(v):a.skip_effect(v);for(const[l,v]of this.#e)l===e?a.unskip_effect(v.effect):a.skip_effect(v.effect);a.oncommit(this.#t),a.ondiscard(this.#i)}else this.#t(a)}}function Q(t,e,r=!1){var a=new Wa(t),i=r?lr:0;function o(s,l){a.ensure(s,l)}Jr(()=>{var s=!1;e((l,v=0)=>{s=!0,o(v,l)}),s||o(-1,null)},i)}const Gl=Symbol("NaN");function Hn(t,e,r){var a=new Wa(t),i=!Kr();Jr(()=>{var o=e();o!==o&&(o=Gl),i&&o!==null&&typeof o=="object"&&(o={}),a.ensure(o,r)})}function Rt(t,e){return e}function Yl(t,e,r){for(var a=[],i=e.length,o,s=e.length,l=0;l<i;l++){let g=e[l];Nr(g,()=>{if(o){if(o.pending.delete(g),o.done.add(g),o.pending.size===0){var k=t.outrogroups;kn(t,Ha(o.done)),k.delete(o),k.size===0&&(t.outrogroups=null)}}else s-=1},!1)}if(s===0){var v=a.length===0&&r!==null&&t.pending.size===0;if(v){var f=r,x=f.parentNode;ml(x),x.append(f),t.items.clear()}kn(t,e,!v)}else o={pending:new Set(e),done:new Set},(t.outrogroups??=new Set).add(o)}function kn(t,e,r=!0){var a;if(t.pending.size>0){a=new Set;for(const s of t.pending.values())for(const l of s)a.add(t.items.get(l).e)}for(var i=0;i<e.length;i++){var o=e[i];if(a?.has(o)){o.f|=tr;const s=document.createDocumentFragment();Fn(o,s)}else kt(e[i],r)}}var oi;function Re(t,e,r,a,i,o=null){var s=t,l=new Map,v=(e&Bi)!==0;if(v){var f=t;s=f.appendChild(nr())}var x=null,g=In(()=>{var b=r();return Fa(b)?b:b==null?[]:Ha(b)}),k=new Map,_=!0;function S(b){if((y.effect.f&wt)===0){y.pending.delete(b);var w=n(g);y.fallback=x,Kl(y,w,s,e,a),x!==null&&(w.length===0?(x.f&tr)===0?Ra(x):(x.f^=tr,ua(x,null,s)):Nr(x,()=>{x=null}))}}function I(b){y.pending.delete(b)}var $=Jr(()=>{for(var b=n(g),w=b.length,A=new Set,N=Le,R=ps(),p=0;p<w;p+=1){var j=b[p],E=a(j,p),C=_?null:l.get(E);C?(C.v&&Ur(C.v,j),C.i&&Ur(C.i,p),R&&N.unskip_effect(C.e)):(C=Jl(l,_?s:oi??=nr(),j,E,p,i,e,r),_||(C.e.f|=tr),l.set(E,C)),A.add(E)}if(w===0&&o&&!x&&(_?x=Et(()=>o(s)):(x=Et(()=>o(oi??=nr())),x.f|=tr)),w>A.size&&Wo(),!_)if(k.set(N,A),R){for(const[q,U]of l)A.has(q)||N.skip_effect(U.e);N.oncommit(S),N.ondiscard(I)}else S(N);n(g)}),y={effect:$,items:l,pending:k,outrogroups:null,fallback:x};_=!1}function ia(t){for(;t!==null&&(t.f&Yt)===0;)t=t.next;return t}function Kl(t,e,r,a,i){var o=(a&Ao)!==0,s=e.length,l=t.items,v=ia(t.effect.first),f,x=null,g,k=[],_=[],S,I,$,y;if(o)for(y=0;y<s;y+=1)S=e[y],I=i(S,y),$=l.get(I).e,($.f&tr)===0&&($.nodes?.a?.measure(),(g??=new Set).add($));for(y=0;y<s;y+=1){if(S=e[y],I=i(S,y),$=l.get(I).e,t.outrogroups!==null)for(const C of t.outrogroups)C.pending.delete($),C.done.delete($);if(($.f&Nt)!==0&&(Ra($),o&&($.nodes?.a?.unfix(),(g??=new Set).delete($))),($.f&tr)!==0)if($.f^=tr,$===v)ua($,null,r);else{var b=x?x.next:v;$===t.effect.last&&(t.effect.last=$.prev),$.prev&&($.prev.next=$.next),$.next&&($.next.prev=$.prev),hr(t,x,$),hr(t,$,b),ua($,b,r),x=$,k=[],_=[],v=ia(x.next);continue}if($!==v){if(f!==void 0&&f.has($)){if(k.length<_.length){var w=_[0],A;x=w.prev;var N=k[0],R=k[k.length-1];for(A=0;A<k.length;A+=1)ua(k[A],w,r);for(A=0;A<_.length;A+=1)f.delete(_[A]);hr(t,N.prev,R.next),hr(t,x,N),hr(t,R,w),v=w,x=R,y-=1,k=[],_=[]}else f.delete($),ua($,v,r),hr(t,$.prev,$.next),hr(t,$,x===null?t.effect.first:x.next),hr(t,x,$),x=$;continue}for(k=[],_=[];v!==null&&v!==$;)(f??=new Set).add(v),_.push(v),v=ia(v.next);if(v===null)continue}($.f&tr)===0&&k.push($),x=$,v=ia($.next)}if(t.outrogroups!==null){for(const C of t.outrogroups)C.pending.size===0&&(kn(t,Ha(C.done)),t.outrogroups?.delete(C));t.outrogroups.size===0&&(t.outrogroups=null)}if(v!==null||f!==void 0){var p=[];if(f!==void 0)for($ of f)($.f&Nt)===0&&p.push($);for(;v!==null;)(v.f&Nt)===0&&v!==t.fallback&&p.push(v),v=ia(v.next);var j=p.length;if(j>0){var E=(a&Bi)!==0&&s===0?r:null;if(o){for(y=0;y<j;y+=1)p[y].nodes?.a?.measure();for(y=0;y<j;y+=1)p[y].nodes?.a?.fix()}Yl(t,p,E)}}o&&Ot(()=>{if(g!==void 0)for($ of g)$.nodes?.a?.apply()})}function Jl(t,e,r,a,i,o,s,l){var v=(s&Po)!==0?(s&zo)===0?gl(r,!1,!1):cr(r):null,f=(s&To)!==0?cr(i):null;return{v,i:f,e:Et(()=>(o(e,v??r,f??i,l),()=>{t.delete(a)}))}}function ua(t,e,r){if(t.nodes)for(var a=t.nodes.start,i=t.nodes.end,o=e&&(e.f&tr)===0?e.nodes.start:r;a!==null;){var s=wa(a);if(o.before(a),a===i)return;a=s}}function hr(t,e,r){e===null?t.effect.first=r:e.next=r,r===null?t.effect.last=e:r.prev=e}function Xl(t,e,r=!1,a=!1,i=!1,o=!1,s){var l=t,v="";if(r)var f=t;O(()=>{var x=je;if(v!==(v=e()??"")){if(r&&!Ji){x.nodes=null,f.innerHTML=v,v!==""&&mr(Bt(f),f.lastChild);return}if(x.nodes!==null&&(ms(x.nodes.start,x.nodes.end),x.nodes=null),v!==""){var g=a?Ki:i?Oo:void 0,k=Qa(a?"svg":i?"math":"template",g);k.innerHTML=v;var _=a||i?k:k.content;if(mr(Bt(_),_.lastChild),a||i)for(;Bt(_);)l.before(Bt(_));else l.before(_)}}})}function ve(t,e,r,a,i,o){if(e.$$host?.$$shadowRoot){const v=Qa("slot");h(t,v);return}var s=e.$$slots?.[r],l=!1;s===!0&&(s=e.children,l=!0),s===void 0||s(t,l?()=>a:a)}function Zl(t,e,...r){var a=new Wa(t);Jr(()=>{const i=e()??null;a.ensure(i,i&&(o=>i(o,...r)))},lr)}const ec=()=>performance.now(),or={tick:t=>requestAnimationFrame(t),now:()=>ec(),tasks:new Set};function Is(){const t=or.now();or.tasks.forEach(e=>{e.c(t)||(or.tasks.delete(e),e.f())}),or.tasks.size!==0&&or.tick(Is)}function tc(t){let e;return or.tasks.size===0&&or.tick(Is),{promise:new Promise(r=>{or.tasks.add(e={c:t,f:r})}),abort(){or.tasks.delete(e)}}}function Aa(t,e){kr(()=>{t.dispatchEvent(new CustomEvent(e))})}function rc(t){if(t==="float")return"cssFloat";if(t==="offset")return"cssOffset";if(t.startsWith("--"))return t;const e=t.split("-");return e.length===1?e[0]:e[0]+e.slice(1).map(r=>r[0].toUpperCase()+r.slice(1)).join("")}function li(t){const e={},r=t.split(";");for(const a of r){const[i,o]=a.split(":");if(!i||o===void 0)break;const s=rc(i.trim());e[s]=o.trim()}return e}const ac=t=>t;function ot(t,e,r,a){var i=(t&Io)!==0,o=(t&Lo)!==0,s=i&&o,l=(t&Ro)!==0,v=s?"both":i?"in":"out",f,x=e.inert,g=e.style.overflow,k,_;function S(){return kr(()=>f??=r()(e,a?.()??{},{direction:v}))}var I={is_global:l,in(){if(e.inert=x,!i){_?.abort(),_?.reset?.();return}o||k?.abort(),k=Sn(e,S(),_,1,()=>{Aa(e,"introstart")},()=>{Aa(e,"introend"),k?.abort(),k=f=void 0,e.style.overflow=g})},out(w){if(!o){w?.(),f=void 0;return}e.inert=!0,_=Sn(e,S(),k,0,()=>{Aa(e,"outrostart")},()=>{Aa(e,"outroend"),w?.()})},stop:()=>{k?.abort(),_?.abort()}},$=je;if(($.nodes.t??=[]).push(I),i&&ja){var y=l;if(!y){for(var b=$.parent;b&&(b.f&lr)!==0;)for(;(b=b.parent)&&(b.f&qt)===0;);y=!b||(b.f&Lr)!==0}y&&Ba(()=>{ur(()=>I.in())})}}function Sn(t,e,r,a,i,o){var s=a===1,l=!1;if(jr(e)){var v;return Ot(()=>{if(!l){var I=e({direction:s?"in":"out"});v=Sn(t,I,r,a,i,o)}}),{abort:()=>{l=!0,v?.abort()},deactivate:()=>v?.deactivate(),reset:()=>v?.reset(),t:()=>v?.t()??1-a}}if(r?.deactivate(),!e?.duration&&!e?.delay)return i(),o(),{abort:qr,deactivate:qr,reset:qr,t:()=>a};const{delay:f=0,css:x,tick:g,easing:k=ac}=e;var _,S=()=>1-a;return Ot(()=>{if(!l){var I=[];if(s&&r===void 0&&(g&&g(0,1),x)){var $=li(x(0,1));I.push($,$)}_=t.animate(I,{duration:f,fill:"forwards"}),_.onfinish=()=>{_.cancel(),i();var y=r?.t()??1-a;r?.abort();var b=a-y,w=e.duration*Math.abs(b),A=[];if(w>0){var N=!1;if(x)for(var R=Math.ceil(w/16.666666666666668),p=0;p<=R;p+=1){var j=y+b*k(p/R),E=li(x(j,1-j));A.push(E),N||=E.overflow==="hidden"}N&&(t.style.overflow="hidden"),S=()=>{var C=_.currentTime;return y+b*k(C/w)},g&&tc(()=>{if(_.playState!=="running")return!1;var C=S();return g(C,1-C),!0})}_=t.animate(A,{duration:w,fill:"forwards"}),_.onfinish=()=>{S=()=>a,g?.(a,1-a),o()}}}}),{abort:()=>{l=!0,_&&(_.cancel(),_.effect=null,_.onfinish=qr)},deactivate:()=>{o=qr},reset:()=>{a===0&&g?.(1,0)},t:()=>S()}}function nc(t,e,r,a,i,o){var s=null,l=t,v=new Wa(l,!1);Jr(()=>{const f=e()||null;var x=Ki;if(f===null){v.ensure(null,null),Pa(!0);return}return v.ensure(f,g=>{if(f){var k=Ji;if(s=Qa(f,x),mr(s,s),a){var _=null,S=s.appendChild(nr());a(s,S),_?.remove()}je.nodes.end=s,k||g.before(s)}}),Pa(!0),()=>{f&&Pa(!1)}},lr),Ua(()=>{Pa(!0)})}function ic(t,e){var r=void 0,a;bs(()=>{r!==(r=e())&&(a&&(kt(a),a=null),r&&(a=Et(()=>{Ba(()=>r(t))})))})}function Ls(t){var e,r,a="";if(typeof t=="string"||typeof t=="number")a+=t;else if(typeof t=="object")if(Array.isArray(t)){var i=t.length;for(e=0;e<i;e++)t[e]&&(r=Ls(t[e]))&&(a&&(a+=" "),a+=r)}else for(r in t)t[r]&&(a&&(a+=" "),a+=r);return a}function sc(){for(var t,e,r=0,a="",i=arguments.length;r<i;r++)(t=arguments[r])&&(e=Ls(t))&&(a&&(a+=" "),a+=e);return a}function oc(t){return typeof t=="object"?sc(t):t??""}const ci=[...` 	
\r\f \v\uFEFF`];function lc(t,e,r){var a=t==null?"":""+t;if(e&&(a=a?a+" "+e:e),r){for(var i of Object.keys(r))if(r[i])a=a?a+" "+i:i;else if(a.length)for(var o=i.length,s=0;(s=a.indexOf(i,s))>=0;){var l=s+o;(s===0||ci.includes(a[s-1]))&&(l===a.length||ci.includes(a[l]))?a=(s===0?"":a.substring(0,s))+a.substring(l+1):s=l}}return a===""?null:a}function di(t,e=!1){var r=e?" !important;":";",a="";for(var i of Object.keys(t)){var o=t[i];o!=null&&o!==""&&(a+=" "+i+": "+o+r)}return a}function an(t){return t[0]!=="-"||t[1]!=="-"?t.toLowerCase():t}function cc(t,e){if(e){var r="",a,i;if(Array.isArray(e)?(a=e[0],i=e[1]):a=e,t){t=String(t).replaceAll(/\/\*.*?\*\//g,"").trim();var o=!1,s=0,l=!1,v=[];a&&v.push(...Object.keys(a).map(an)),i&&v.push(...Object.keys(i).map(an));var f=0,x=-1;const I=t.length;for(var g=0;g<I;g++){var k=t[g];if(l?k==="/"&&t[g-1]==="*"&&(l=!1):o?o===k&&(o=!1):k==="/"&&t[g+1]==="*"?l=!0:k==='"'||k==="'"?o=k:k==="("?s++:k===")"&&s--,!l&&o===!1&&s===0){if(k===":"&&x===-1)x=g;else if(k===";"||g===I-1){if(x!==-1){var _=an(t.substring(f,x).trim());if(!v.includes(_)){k!==";"&&g++;var S=t.substring(f,g).trim();r+=" "+S+";"}}f=g+1,x=-1}}}}return a&&(r+=di(a)),i&&(r+=di(i,!0)),r=r.trim(),r===""?null:r}return t==null?null:String(t)}function ee(t,e,r,a,i,o){var s=t[pn];if(s!==r||s===void 0){var l=lc(r,a,o);l==null?t.removeAttribute("class"):e?t.className=l:t.setAttribute("class",l),t[pn]=r}else if(o&&i!==o)for(var v in o){var f=!!o[v];(i==null||f!==!!i[v])&&t.classList.toggle(v,f)}return o}function nn(t,e={},r,a){for(var i in r){var o=r[i];e[i]!==o&&(r[i]==null?t.style.removeProperty(i):t.style.setProperty(i,o,a))}}function Ke(t,e,r,a){var i=t[_n];if(i!==e){var o=cc(e,a);o==null?t.removeAttribute("style"):t.style.cssText=o,t[_n]=e}else a&&(Array.isArray(a)?(nn(t,r?.[0],a[0]),nn(t,r?.[1],a[1],"important")):nn(t,r,a));return a}function Rs(t,e){e?t.hasAttribute("selected")||t.setAttribute("selected",""):t.removeAttribute("selected")}function ui(t,e){var r=!("__defaultValue"in t);!r&&t.__defaultValue===e||(t.__defaultValue=e,js(t,!r||"__value"in t))}function js(t,e){var r=t.__defaultValue,a=t.multiple,i=a?r??[]:null;if(!(a&&!Fa(i))){var o=t.selectedIndex,s=e&&a?new Set(t.selectedOptions):null;for(var l of t.options){var v=Pn(l);Rs(l,a?i.includes(v):us(v,r))}if(e)if(s!==null)for(l of t.options){var f=s.has(l);l.selected!==f&&(l.selected=f)}else t.selectedIndex!==o&&(t.selectedIndex=o)}}function $n(t,e,r=!1){if(t.multiple){if(e==null)return;if(!Fa(e))return Ho();for(var a of t.options)a.selected=e.includes(Pn(a));return}for(a of t.options){var i=Pn(a);if(us(i,e)){a.selected=!0;return}}(!r||e!==void 0)&&(t.selectedIndex=-1)}function dc(t){var e=new MutationObserver(r=>{r.every(uc)||("__defaultValue"in t&&js(t,!1),"__value"in t&&$n(t,t.__value))});e.observe(t,{childList:!0,subtree:!0,attributes:!0,attributeFilter:["value"]}),Ua(()=>{e.disconnect()})}function Pn(t){return"__value"in t?t.__value:t.value}function uc(t){if(t.target.closest("selectedcontent")!==null)return!0;if(t.type==="childList"){var e=[...t.addedNodes,...t.removedNodes];return e.length>0&&e.every(r=>r.nodeName==="SELECTEDCONTENT")}return!1}const sa=Symbol("class"),oa=Symbol("style"),qs=Symbol("is custom element"),Os=Symbol("is html"),vc=Va?"input":"INPUT",fc=Va?"option":"OPTION",Ds=Va?"select":"SELECT",Fs=Va?"progress":"PROGRESS";function Tn(t,e){var r=Vn(t);r.value===(r.value=e??void 0)||t.value===e&&(e!==0||t.nodeName!==Fs)||(t.value=e??"")}function le(t,e,r,a){var i=Vn(t),o=i[e];o!==(i[e]=r)&&(e==="loading"&&(t[$o]=r),r==null?t.removeAttribute(e):typeof r!="string"&&Hs(t).has(e)?t[e]=r:t.setAttribute(e,r))}function hc(t,e,r,a,i=!1,o=!1){var s=Vn(t),l=s[qs],v=!s[Os],f=e||{},x=t.nodeName===fc,g=t.nodeName===Ds;try{for(var k in e)!(k in r)&&k[0]+k[1]!=="$$"&&(r[k]=null);r.class?r.class=oc(r.class):(a||r[sa])&&(r.class=null),r[oa]&&(r.style??=null);var _=Hs(t);if(t.nodeName===vc&&"type"in r&&("value"in r||"__value"in r)){var S=r.type;(S!==f.type||S===void 0&&t.hasAttribute("type"))&&(f.type=S,le(t,"type",S,o))}for(const R in r){let p=r[R];if(x&&R==="value"&&p==null){t.value=t.__value="",f[R]=p;continue}if(R==="class"){var I=t.namespaceURI==="http://www.w3.org/1999/xhtml";ee(t,I,p,a,e?.[sa],r[sa]),f[R]=p,f[sa]=r[sa];continue}if(R==="style"){Ke(t,p,e?.[oa],r[oa]),f[R]=p,f[oa]=r[oa];continue}var $=f[R];if(!(p===$&&!(p===void 0&&t.hasAttribute(R)))){f[R]=p;var y=R[0]+R[1];if(y!=="$$")if(y==="on"){const j={},E="$$"+R;let C=R.slice(2);var b=El(C);if(Nl(C)&&(C=C.slice(0,-7),j.capture=!0),!b&&$){if(p!=null)continue;t.removeEventListener(C,f[E],j),f[E]=null}if(b)M(C,t,p),Ve([C]);else if(p!=null){let q=function(U){f[R].call(this,U)};var N=q;f[E]=Es(C,t,q,j)}}else if(R==="style")le(t,R,p);else if(R==="autofocus")ol(t,!!p);else if(!l&&(R==="__value"||R==="value"&&p!=null))t.__value=p,($==null||t.value!==p||p===0&&t.nodeName===Fs)&&(t.value=p);else if(R==="selected"&&x)Rs(t,p);else{var w=R;v||(w=Il(w));var A=w==="defaultValue"||w==="defaultChecked";if(g&&w==="defaultValue")continue;if(p==null&&!l&&!A)if(s[R]=null,w==="value"||w==="checked"){let j=t;const E=e===void 0;if(w==="value"){let C=j.defaultValue;j.removeAttribute(w),j.defaultValue=C,j.value=j.__value=E?C:null}else{let C=j.defaultChecked;j.removeAttribute(w),j.defaultChecked=C,j.checked=E?C:!1}}else t.removeAttribute(R);else A||(l||typeof p!="string")&&_.has(w)?(t[w]=p,w in s&&(s[w]=ft)):typeof p!="function"&&le(t,w,p,o)}}}}finally{}return f}function vi(t,e,r=[],a=[],i=[],o,s=!1,l=!1){rs(i,r,a,v=>{var f=void 0,x={},g=t.nodeName===Ds,k=!1;if(bs(()=>{var S=e(...v.map(n)),I=hc(t,f,S,o,s,l);if(k&&g){var $=t;"defaultValue"in S&&ui($,S.defaultValue),"value"in S&&$n($,S.value)}for(let b of Object.getOwnPropertySymbols(x))S[b]||kt(x[b]);for(let b of Object.getOwnPropertySymbols(S)){var y=S[b];b.description===Do&&(!f||y!==f[b])&&(x[b]&&kt(x[b]),x[b]=Et(()=>ic(t,()=>y))),I[b]=y}f=I}),g){var _=t;Ba(()=>{var S=f;"defaultValue"in S&&ui(_,S.defaultValue),$n(_,S.value,!0),dc(_)})}k=!0})}function Vn(t){return t[Ui]??={[qs]:t.nodeName.includes("-"),[Os]:t.namespaceURI===Yi}}var fi=new Map;function Hs(t){var e=t.getAttribute("is")||t.nodeName,r=fi.get(e);if(r)return r;fi.set(e,r=new Set);for(var a,i=t,o=Element.prototype;o!==i;){a=Di(i);for(var s in a)a[s].set&&s!=="innerHTML"&&s!=="textContent"&&s!=="innerText"&&r.add(s);i=zn(i)}return r}function qa(t,e,r=e){var a=new WeakSet;cl(t,"input",async i=>{var o=i?t.defaultValue:t.value;if(o=sn(t)?on(o):o,r(o),Le!==null&&a.add(Le),await zl(),o!==(o=e())){var s=t.selectionStart,l=t.selectionEnd,v=t.value.length;if(t.value=o??"",l!==null){var f=t.value.length;s===l&&l===v&&f>v?(t.selectionStart=f,t.selectionEnd=f):(t.selectionStart=s,t.selectionEnd=Math.min(l,f))}}}),ur(e)==null&&t.value&&(r(sn(t)?on(t.value):t.value),Le!==null&&a.add(Le)),On(()=>{var i=e();if(t===document.activeElement){var o=Le;if(a.has(o))return}sn(t)&&i===on(t.value)||t.type==="date"&&!i&&!t.value||i!==t.value&&(t.value=i??"")})}function sn(t){var e=t.type;return e==="number"||e==="range"}function on(t){return t===""?null:+t}function ln(t,e){return t===e||t?.[rr]===e}function Xr(t=En(),e,r,a){var i=it.r,o=je;return Ba(()=>{var s,l;return On(()=>{s=l,l=a?.()||[],ur(()=>{ln(r(...l),t)||(e(t,...l),s&&ln(r(...s),t)&&e(null,...s))})}),()=>{let v=o;for(;v!==i&&v.parent!==null&&v.parent.f&_a;)v=v.parent;const f=()=>{l&&ln(r(...l),t)&&e(null,...l)},x=v.teardown;v.teardown=()=>{f(),x?.()}}}),t}function Qn(t=!1){const e=it,r=e.l.u;if(!r)return;let a=()=>Pr(e.s);if(t){let i=0,o={};const s=Qr(()=>{let l=!1;const v=e.s;for(const f in v)v[f]!==o[f]&&(o[f]=v[f],l=!0);return l&&i++,i});a=()=>n(s)}r.b.length&&kl(()=>{hi(e,a),fn(r.b)}),$t(()=>{const i=ur(()=>r.m.map(wo));return()=>{for(const o of i)typeof o=="function"&&o()}}),r.a.length&&$t(()=>{hi(e,a),fn(r.a)})}function hi(t,e){if(t.l.s)for(const r of t.l.s)n(r);e()}function pc(t){var e=cr(0);return function(){return arguments.length===1?(z(e,n(e)+1),arguments[0]):(n(e),t())}}let za=!1;function _c(t){var e=za;try{return za=!1,[t(),za]}finally{za=e}}const gc={get(t,e){if(!t.exclude.includes(e))return n(t.version),e in t.special?t.special[e]():t.props[e]},set(t,e,r){if(!(e in t.special)){var a=je;try{Ft(t.parent_effect),t.special[e]=et({get[e](){return t.props[e]}},e,Wi)}finally{Ft(a)}}return t.special[e](r),ri(t.version),!0},getOwnPropertyDescriptor(t,e){if(!t.exclude.includes(e)&&e in t.props)return{enumerable:!0,configurable:!0,value:t.props[e]}},deleteProperty(t,e){return t.exclude.includes(e)||(t.exclude.push(e),ri(t.version)),!0},has(t,e){return t.exclude.includes(e)?!1:e in t.props},ownKeys(t){return Reflect.ownKeys(t.props).filter(e=>!t.exclude.includes(e))}};function ue(t,e){return new Proxy({props:t,exclude:e,special:{},version:cr(0),parent_effect:je},gc)}const bc={get(t,e){let r=t.props.length;for(;r--;){let a=t.props[r];if(jr(a)&&(a=a()),typeof a=="object"&&a!==null&&e in a)return a[e]}},set(t,e,r){let a=t.props.length;for(;a--;){let i=t.props[a];jr(i)&&(i=i());const o=_r(i,e);if(o&&o.set)return o.set(r),!0}return!1},getOwnPropertyDescriptor(t,e){let r=t.props.length;for(;r--;){let a=t.props[r];if(jr(a)&&(a=a()),typeof a=="object"&&a!==null&&e in a){const i=_r(a,e);return i&&!i.configurable&&(i.configurable=!0),i}}},has(t,e){if(e===rr||e===Qi)return!1;for(let r of t.props)if(jr(r)&&(r=r()),r!=null&&e in r)return!0;return!1},ownKeys(t){const e=[];for(let r of t.props)if(jr(r)&&(r=r()),!!r){for(const a in r)e.includes(a)||e.push(a);for(const a of Object.getOwnPropertySymbols(r))e.includes(a)||e.push(a)}return e}};function fe(...t){return new Proxy({props:t},bc)}function et(t,e,r,a){var i=!Yr||(r&Mo)!==0,o=(r&Eo)!==0,s=(r&Co)!==0,l=a,v=!0,f=void 0,x=()=>s&&i?(f??=Qr(a),n(f)):(v&&(v=!1,l=s?ur(a):a),l);let g;if(o){var k=rr in t||Qi in t;g=_r(t,e)?.set??(k&&e in t?A=>t[e]=A:void 0)}var _,S=!1;o?[_,S]=_c(()=>t[e]):_=t[e],_===void 0&&a!==void 0&&(_=x(),g&&(i&&Xo(),g(_)));var I;if(i?I=()=>{var A=t[e];return A===void 0?x():(v=!0,A)}:I=()=>{var A=t[e];return A!==void 0&&(l=void 0),A===void 0?l:A},i&&(r&Wi)===0)return I;if(g){var $=t.$$legacy;return(function(A,N){return arguments.length>0?((!i||!N||$||S)&&g(N?I():A),A):I()})}var y=!1,b=((r&No)!==0?Qr:In)(()=>(y=!1,I()));o&&n(b);var w=je;return(function(A,N){if(arguments.length>0){const R=N?n(b):i&&o?Je(A):A;return z(b,R),y=!0,l!==void 0&&(l=R),A}return dr&&y||(w.f&wt)!==0?b.v:n(b)})}function Sa(t){it===null&&Uo(),Yr&&it.l!==null?xc(it).m.push(t):$t(()=>{const e=ur(t);if(typeof e=="function")return e})}function xc(t){var e=t.l;return e.u??={a:[],b:[],m:[]}}const mc="5";typeof window<"u"&&((window.__svelte??={}).v??=new Set).add(mc);nl();/**
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
 */const yc={xmlns:"http://www.w3.org/2000/svg",width:24,height:24,viewBox:"0 0 24 24",fill:"none",stroke:"currentColor","stroke-width":2,"stroke-linecap":"round","stroke-linejoin":"round"};/**
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
 */const wc=t=>{for(const e in t)if(e.startsWith("aria-")||e==="role"||e==="title")return!0;return!1};/**
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
 */const pi=(...t)=>t.filter((e,r,a)=>!!e&&e.trim()!==""&&a.indexOf(e)===r).join(" ").trim();var kc=Dl("<svg><!><!></svg>");function he(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]),a=ue(r,["name","color","size","strokeWidth","absoluteStrokeWidth","iconNode"]);Ae(e,!1);let i=et(e,"name",8,void 0),o=et(e,"color",8,"currentColor"),s=et(e,"size",8,24),l=et(e,"strokeWidth",8,2),v=et(e,"absoluteStrokeWidth",8,!1),f=et(e,"iconNode",24,()=>[]);Qn();var x=kc();vi(x,(_,S,I)=>({...yc,..._,...a,width:s(),height:s(),stroke:o(),"stroke-width":S,class:I}),[()=>wc(a)?void 0:{"aria-hidden":"true"},()=>(Pr(v()),Pr(l()),Pr(s()),ur(()=>v()?Number(l())*24/Number(s()):l())),()=>(Pr(pi),Pr(i()),Pr(r),ur(()=>pi("lucide-icon","lucide",i()?`lucide-${i()}`:"",r.class)))]);var g=u(x);Re(g,1,f,Rt,(_,S)=>{var I=J(()=>ko(n(S),2));let $=()=>n(I)[0],y=()=>n(I)[1];var b=se(),w=K(b);nc(w,$,!0,(A,N)=>{vi(A,()=>({...y()}))}),h(_,b)});var k=d(g);ve(k,e,"default",{}),h(t,x),ze()}function Ga(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"m12 19-7-7 7-7"}],["path",{d:"M19 12H5"}]];he(t,fe({name:"arrow-left"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Sc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M5 12h14"}],["path",{d:"m12 5 7 7-7 7"}]];he(t,fe({name:"arrow-right"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function $c(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"m21 16-4 4-4-4"}],["path",{d:"M17 20V4"}],["path",{d:"m3 8 4-4 4 4"}],["path",{d:"M7 4v16"}]];he(t,fe({name:"arrow-up-down"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Pc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M7 17V7h10"}],["path",{d:"M17 17 7 7"}]];he(t,fe({name:"arrow-up-left"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Vs(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M19 17h2c.6 0 1-.4 1-1v-3c0-.9-.7-1.7-1.5-1.9C18.7 10.6 16 10 16 10s-1.3-1.4-2.2-2.3c-.5-.4-1.1-.7-1.8-.7H5c-.6 0-1.1.4-1.4.9l-1.4 2.9A3.7 3.7 0 0 0 2 12v4c0 .6.4 1 1 1h2"}],["circle",{cx:"7",cy:"17",r:"2"}],["path",{d:"M9 17h6"}],["circle",{cx:"17",cy:"17",r:"2"}]];he(t,fe({name:"car"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Qs(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M20 6 9 17l-5-5"}]];he(t,fe({name:"check"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Tc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"m6 9 6 6 6-6"}]];he(t,fe({name:"chevron-down"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function _i(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"m15 18-6-6 6-6"}]];he(t,fe({name:"chevron-left"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function An(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"m9 18 6-6-6-6"}]];he(t,fe({name:"chevron-right"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Ya(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["circle",{cx:"12",cy:"12",r:"10"}],["path",{d:"M12 6v6h4"}]];he(t,fe({name:"clock-3"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Ac(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["circle",{cx:"12",cy:"12",r:"10"}],["path",{d:"M12 6v6l4 2"}]];he(t,fe({name:"clock"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function zc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"}],["path",{d:"M16 14v6"}],["path",{d:"M8 14v6"}],["path",{d:"M12 16v6"}]];he(t,fe({name:"cloud-rain"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Nc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M10 2v2"}],["path",{d:"M14 2v2"}],["path",{d:"M16 8a1 1 0 0 1 1 1v8a4 4 0 0 1-4 4H7a4 4 0 0 1-4-4V9a1 1 0 0 1 1-1h14a4 4 0 1 1 0 8h-1"}],["path",{d:"M6 2v2"}]];he(t,fe({name:"coffee"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function xa(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["circle",{cx:"12",cy:"12",r:"10"}],["path",{d:"m16.24 7.76-1.804 5.411a2 2 0 0 1-1.265 1.265L7.76 16.24l1.804-5.411a2 2 0 0 1 1.265-1.265z"}]];he(t,fe({name:"compass"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Mc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["rect",{width:"14",height:"14",x:"8",y:"8",rx:"2",ry:"2"}],["path",{d:"M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"}]];he(t,fe({name:"copy"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Us(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["circle",{cx:"12",cy:"12",r:"10"}],["path",{d:"M6 12c0-1.7.7-3.2 1.8-4.2"}],["circle",{cx:"12",cy:"12",r:"2"}],["path",{d:"M18 12c0 1.7-.7 3.2-1.8 4.2"}]];he(t,fe({name:"disc-3"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Ec(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M17.596 12.768a2 2 0 1 0 2.829-2.829l-1.768-1.767a2 2 0 0 0 2.828-2.829l-2.828-2.828a2 2 0 0 0-2.829 2.828l-1.767-1.768a2 2 0 1 0-2.829 2.829z"}],["path",{d:"m2.5 21.5 1.4-1.4"}],["path",{d:"m20.1 3.9 1.4-1.4"}],["path",{d:"M5.343 21.485a2 2 0 1 0 2.829-2.828l1.767 1.768a2 2 0 1 0 2.829-2.829l-6.364-6.364a2 2 0 1 0-2.829 2.829l1.768 1.767a2 2 0 0 0-2.828 2.829z"}],["path",{d:"m9.6 14.4 4.8-4.8"}]];he(t,fe({name:"dumbbell"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Cc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M12 3q1 4 4 6.5t3 5.5a1 1 0 0 1-14 0 5 5 0 0 1 1-3 1 1 0 0 0 5 0c0-2-1.5-3-1.5-5q0-2 2.5-4"}]];he(t,fe({name:"flame"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Bs(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["circle",{cx:"12",cy:"12",r:"10"}],["path",{d:"M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"}],["path",{d:"M2 12h20"}]];he(t,fe({name:"globe"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Ic(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["circle",{cx:"9",cy:"12",r:"1"}],["circle",{cx:"9",cy:"5",r:"1"}],["circle",{cx:"9",cy:"19",r:"1"}],["circle",{cx:"15",cy:"12",r:"1"}],["circle",{cx:"15",cy:"5",r:"1"}],["circle",{cx:"15",cy:"19",r:"1"}]];he(t,fe({name:"grip-vertical"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Lc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M10 16h.01"}],["path",{d:"M2.212 11.577a2 2 0 0 0-.212.896V18a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-5.527a2 2 0 0 0-.212-.896L18.55 5.11A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"}],["path",{d:"M21.946 12.013H2.054"}],["path",{d:"M6 16h.01"}]];he(t,fe({name:"hard-drive"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Ws(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M3 14h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-7a9 9 0 0 1 18 0v7a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3"}]];he(t,fe({name:"headphones"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function gt(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M2 9.5a5.5 5.5 0 0 1 9.591-3.676.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5l-5.492 5.313a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5"}]];he(t,fe({name:"heart"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function gi(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8"}],["path",{d:"M3 10a2 2 0 0 1 .709-1.528l7-6a2 2 0 0 1 2.582 0l7 6A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"}]];he(t,fe({name:"house"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function bi(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"m16 6 4 14"}],["path",{d:"M12 6v14"}],["path",{d:"M8 8v12"}],["path",{d:"M4 4v16"}]];he(t,fe({name:"library"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Gs(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M16 5H3"}],["path",{d:"M11 12H3"}],["path",{d:"M11 19H3"}],["path",{d:"M21 16V5"}],["circle",{cx:"18",cy:"16",r:"3"}]];he(t,fe({name:"list-music"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Zr(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M16 5H3"}],["path",{d:"M11 12H3"}],["path",{d:"M16 19H3"}],["path",{d:"M18 9v6"}],["path",{d:"M21 12h-6"}]];he(t,fe({name:"list-plus"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Ys(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["line",{x1:"2",x2:"5",y1:"12",y2:"12"}],["line",{x1:"19",x2:"22",y1:"12",y2:"12"}],["line",{x1:"12",x2:"12",y1:"2",y2:"5"}],["line",{x1:"12",x2:"12",y1:"19",y2:"22"}],["circle",{cx:"12",cy:"12",r:"7"}],["circle",{cx:"12",cy:"12",r:"3"}]];he(t,fe({name:"locate-fixed"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Rc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"m10 17 5-5-5-5"}],["path",{d:"M15 12H3"}],["path",{d:"M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"}]];he(t,fe({name:"log-in"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function jc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"m16 17 5-5-5-5"}],["path",{d:"M21 12H9"}],["path",{d:"M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"}]];he(t,fe({name:"log-out"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function qc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M14 14a2 2 0 0 0 2-2V8h-2"}],["path",{d:"M22 17a2 2 0 0 1-2 2H6.828a2 2 0 0 0-1.414.586l-2.202 2.202A.71.71 0 0 1 2 21.286V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z"}],["path",{d:"M8 14a2 2 0 0 0 2-2V8H8"}]];he(t,fe({name:"message-square-quote"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Oc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["rect",{width:"20",height:"14",x:"2",y:"3",rx:"2"}],["line",{x1:"8",x2:"16",y1:"21",y2:"21"}],["line",{x1:"12",x2:"12",y1:"17",y2:"21"}]];he(t,fe({name:"monitor"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Un(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M20.985 12.486a9 9 0 1 1-9.473-9.472c.405-.022.617.46.402.803a6 6 0 0 0 8.268 8.268c.344-.215.825-.004.803.401"}]];he(t,fe({name:"moon"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function ea(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M9 18V5l12-2v13"}],["circle",{cx:"6",cy:"18",r:"3"}],["circle",{cx:"18",cy:"16",r:"3"}]];he(t,fe({name:"music"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Dc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["rect",{width:"18",height:"18",x:"3",y:"3",rx:"2"}],["path",{d:"M9 3v18"}]];he(t,fe({name:"panel-left"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Fc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["rect",{width:"18",height:"18",x:"3",y:"3",rx:"2"}],["path",{d:"M15 3v18"}]];he(t,fe({name:"panel-right"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Bn(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M5.8 11.3 2 22l10.7-3.79"}],["path",{d:"M4 3h.01"}],["path",{d:"M22 8h.01"}],["path",{d:"M15 2h.01"}],["path",{d:"M22 20h.01"}],["path",{d:"m22 2-2.24.75a2.9 2.9 0 0 0-1.96 3.12c.1.86-.57 1.63-1.45 1.63h-.38c-.86 0-1.6.6-1.76 1.44L14 10"}],["path",{d:"m22 13-.82-.33c-.86-.34-1.82.2-1.98 1.11c-.11.7-.72 1.22-1.43 1.22H17"}],["path",{d:"m11 2 .33.82c.34.86-.2 1.82-1.11 1.98C9.52 4.9 9 5.52 9 6.23V7"}],["path",{d:"M11 13c1.93 1.93 2.83 4.17 2 5-.83.83-3.07-.07-5-2-1.93-1.93-2.83-4.17-2-5 .83-.83 3.07.07 5 2Z"}]];he(t,fe({name:"party-popper"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Wn(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["rect",{x:"14",y:"3",width:"5",height:"18",rx:"1"}],["rect",{x:"5",y:"3",width:"5",height:"18",rx:"1"}]];he(t,fe({name:"pause"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Ks(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z"}],["path",{d:"m15 5 4 4"}]];he(t,fe({name:"pencil"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function bt(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M5 5a2 2 0 0 1 3.008-1.728l11.997 6.998a2 2 0 0 1 .003 3.458l-12 7A2 2 0 0 1 5 19z"}]];he(t,fe({name:"play"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function yr(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M5 12h14"}],["path",{d:"M12 5v14"}]];he(t,fe({name:"plus"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function $a(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M16.247 7.761a6 6 0 0 1 0 8.478"}],["path",{d:"M19.075 4.933a10 10 0 0 1 0 14.134"}],["path",{d:"M4.925 19.067a10 10 0 0 1 0-14.134"}],["path",{d:"M7.753 16.239a6 6 0 0 1 0-8.478"}],["circle",{cx:"12",cy:"12",r:"2"}]];he(t,fe({name:"radio"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Hc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"}],["path",{d:"M21 3v5h-5"}],["path",{d:"M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"}],["path",{d:"M8 16H3v5"}]];he(t,fe({name:"refresh-cw"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Js(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"m17 2 4 4-4 4"}],["path",{d:"M3 11v-1a4 4 0 0 1 4-4h14"}],["path",{d:"m7 22-4-4 4-4"}],["path",{d:"M21 13v1a4 4 0 0 1-4 4H3"}]];he(t,fe({name:"repeat"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function xi(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"m21 21-4.34-4.34"}],["circle",{cx:"11",cy:"11",r:"8"}]];he(t,fe({name:"search"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Vc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M9.671 4.136a2.34 2.34 0 0 1 4.659 0 2.34 2.34 0 0 0 3.319 1.915 2.34 2.34 0 0 1 2.33 4.033 2.34 2.34 0 0 0 0 3.831 2.34 2.34 0 0 1-2.33 4.033 2.34 2.34 0 0 0-3.319 1.915 2.34 2.34 0 0 1-4.659 0 2.34 2.34 0 0 0-3.32-1.915 2.34 2.34 0 0 1-2.33-4.033 2.34 2.34 0 0 0 0-3.831A2.34 2.34 0 0 1 6.35 6.051a2.34 2.34 0 0 0 3.319-1.915"}],["circle",{cx:"12",cy:"12",r:"3"}]];he(t,fe({name:"settings"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function ta(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"m18 14 4 4-4 4"}],["path",{d:"m18 2 4 4-4 4"}],["path",{d:"M2 18h1.973a4 4 0 0 0 3.3-1.7l5.454-8.6a4 4 0 0 1 3.3-1.7H22"}],["path",{d:"M2 6h1.972a4 4 0 0 1 3.6 2.2"}],["path",{d:"M22 18h-6.041a4 4 0 0 1-3.3-1.8l-.359-.45"}]];he(t,fe({name:"shuffle"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Gn(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M17.971 4.285A2 2 0 0 1 21 6v12a2 2 0 0 1-3.029 1.715l-9.997-5.998a2 2 0 0 1-.003-3.432z"}],["path",{d:"M3 20V4"}]];he(t,fe({name:"skip-back"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Yn(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M21 4v16"}],["path",{d:"M6.029 4.285A2 2 0 0 0 3 6v12a2 2 0 0 0 3.029 1.715l9.997-5.998a2 2 0 0 0 .003-3.432z"}]];he(t,fe({name:"skip-forward"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Qc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M11.017 2.814a1 1 0 0 1 1.966 0l1.051 5.558a2 2 0 0 0 1.594 1.594l5.558 1.051a1 1 0 0 1 0 1.966l-5.558 1.051a2 2 0 0 0-1.594 1.594l-1.051 5.558a1 1 0 0 1-1.966 0l-1.051-5.558a2 2 0 0 0-1.594-1.594l-5.558-1.051a1 1 0 0 1 0-1.966l5.558-1.051a2 2 0 0 0 1.594-1.594z"}],["path",{d:"M20 2v4"}],["path",{d:"M22 4h-4"}],["circle",{cx:"4",cy:"20",r:"2"}]];he(t,fe({name:"sparkles"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Uc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z"}]];he(t,fe({name:"star"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Xs(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["circle",{cx:"12",cy:"12",r:"4"}],["path",{d:"M12 2v2"}],["path",{d:"M12 20v2"}],["path",{d:"m4.93 4.93 1.41 1.41"}],["path",{d:"m17.66 17.66 1.41 1.41"}],["path",{d:"M2 12h2"}],["path",{d:"M20 12h2"}],["path",{d:"m6.34 17.66-1.41 1.41"}],["path",{d:"m19.07 4.93-1.41 1.41"}]];he(t,fe({name:"sun"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Bc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["circle",{cx:"12",cy:"12",r:"10"}],["circle",{cx:"12",cy:"12",r:"6"}],["circle",{cx:"12",cy:"12",r:"2"}]];he(t,fe({name:"target"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Wc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["line",{x1:"10",x2:"14",y1:"2",y2:"2"}],["line",{x1:"12",x2:"15",y1:"14",y2:"11"}],["circle",{cx:"12",cy:"14",r:"8"}]];he(t,fe({name:"timer"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Kn(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M10 11v6"}],["path",{d:"M14 11v6"}],["path",{d:"M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"}],["path",{d:"M3 6h18"}],["path",{d:"M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"}]];he(t,fe({name:"trash-2"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Gc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M16 7h6v6"}],["path",{d:"m22 7-8.5 8.5-5-5L2 17"}]];he(t,fe({name:"trending-up"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function mi(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M11 4.702a.705.705 0 0 0-1.203-.498L6.413 7.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.705.705 0 0 0 11 19.298z"}],["path",{d:"M16 9a5 5 0 0 1 0 6"}]];he(t,fe({name:"volume-1"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function vr(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M11 4.702a.705.705 0 0 0-1.203-.498L6.413 7.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.705.705 0 0 0 11 19.298z"}],["path",{d:"M16 9a5 5 0 0 1 0 6"}],["path",{d:"M19.364 18.364a9 9 0 0 0 0-12.728"}]];he(t,fe({name:"volume-2"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function yi(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M11 4.702a.705.705 0 0 0-1.203-.498L6.413 7.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.705.705 0 0 0 11 19.298z"}],["line",{x1:"22",x2:"16",y1:"9",y2:"15"}],["line",{x1:"16",x2:"22",y1:"9",y2:"15"}]];he(t,fe({name:"volume-x"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Yc(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M12 20h.01"}],["path",{d:"M2 8.82a15 15 0 0 1 20 0"}],["path",{d:"M5 12.859a10 10 0 0 1 14 0"}],["path",{d:"M8.5 16.429a5 5 0 0 1 7 0"}]];he(t,fe({name:"wifi"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Cr(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M18 6 6 18"}],["path",{d:"m6 6 12 12"}]];he(t,fe({name:"x"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function Zs(t,e){const r=ue(e,["children","$$slots","$$events","$$legacy"]);/**
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
 */const a=[["path",{d:"M4 14a1 1 0 0 1-.78-1.63l9.9-10.2a.5.5 0 0 1 .86.46l-1.92 6.02A1 1 0 0 0 13 10h7a1 1 0 0 1 .78 1.63l-9.9 10.2a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14z"}]];he(t,fe({name:"zap"},()=>r,{get iconNode(){return a},children:(i,o)=>{var s=se(),l=K(s);ve(l,e,"default",{}),h(i,s)},$$slots:{default:!0}}))}function At(t){const e=t-1;return e*e*e+1}const Kc=t=>t;function eo(t){const e=t-1;return e*e*e+1}function wi(t){const e=typeof t=="string"&&t.match(/^\s*(-?[\d.]+)([^\s]*)\s*$/);return e?[parseFloat(e[1]),e[2]||"px"]:[t,"px"]}function wr(t,{delay:e=0,duration:r=400,easing:a=Kc}={}){const i=+getComputedStyle(t).opacity;return{delay:e,duration:r,easing:a,css:o=>`opacity: ${o*i}`}}function pr(t,{delay:e=0,duration:r=400,easing:a=eo,x:i=0,y:o=0,opacity:s=0}={}){const l=getComputedStyle(t),v=+l.opacity,f=l.transform==="none"?"":l.transform,x=v*(1-s),[g,k]=wi(i),[_,S]=wi(o);return{delay:e,duration:r,easing:a,css:(I,$)=>`
			transform: ${f} translate(${(1-I)*g}${k}, ${(1-I)*_}${S});
			opacity: ${v-x*$}`}}function Ir(t,{delay:e=0,duration:r=400,easing:a=eo,start:i=0,opacity:o=0}={}){const s=getComputedStyle(t),l=+s.opacity,v=s.transform==="none"?"":s.transform,f=1-i,x=l*(1-o);return{delay:e,duration:r,easing:a,css:(g,k)=>`
			transform: ${v} scale(${1-f*k});
			opacity: ${l-x*k}
		`}}const Jc="modulepreload",Xc=function(t){return"/"+t},ki={},Zc=function(e,r,a){let i=Promise.resolve();if(r&&r.length>0){let s=function(f){return Promise.all(f.map(x=>Promise.resolve(x).then(g=>({status:"fulfilled",value:g}),g=>({status:"rejected",reason:g}))))};document.getElementsByTagName("link");const l=document.querySelector("meta[property=csp-nonce]"),v=l?.nonce||l?.getAttribute("nonce");i=s(r.map(f=>{if(f=Xc(f),f in ki)return;ki[f]=!0;const x=f.endsWith(".css"),g=x?'[rel="stylesheet"]':"";if(document.querySelector(`link[href="${f}"]${g}`))return;const k=document.createElement("link");if(k.rel=x?"stylesheet":Jc,x||(k.as="script"),k.crossOrigin="",k.href=f,v&&k.setAttribute("nonce",v),document.head.appendChild(k),x)return new Promise((_,S)=>{k.addEventListener("load",_),k.addEventListener("error",()=>S(new Error(`Unable to preload CSS for ${f}`)))})}))}function o(s){const l=new Event("vite:preloadError",{cancelable:!0});if(l.payload=s,window.dispatchEvent(l),!l.defaultPrevented)throw s}return i.then(s=>{for(const l of s||[])l.status==="rejected"&&o(l.reason);return e().catch(o)})};function Ue(){return typeof window<"u"&&!!window.__TAURI_INTERNALS__}async function Ge(t,e){if(!Ue())throw new Error(`Tauri runtime not available for command "${t}"`);const{invoke:r}=await Zc(async()=>{const{invoke:a}=await Promise.resolve().then(()=>td);return{invoke:a}},void 0);return r(t,e)}const Ne={loadInitialState:()=>Ge("load_initial_state"),saveSession:t=>Ge("save_session",{session:t}),saveFavorites:t=>Ge("save_favorites_cmd",{favorites:t}),saveRecentTracks:t=>Ge("save_recent_tracks_cmd",{tracks:t}),saveUserPlaylists:t=>Ge("save_user_playlists_cmd",{playlists:t}),saveSearchHistory:t=>Ge("save_search_history_cmd",{history:t}),getHomeFeed:(t=!1)=>Ge("get_home_feed",{forceRefresh:t}),getGenres:(t=!1)=>Ge("get_genres",{forceRefresh:t}),getGenrePlaylists:t=>Ge("get_genre_playlists",{genreId:t}),searchAll:t=>Ge("search_all",{query:t}),getSearchSuggestions:t=>Ge("get_search_suggestions",{query:t}),getArtist:(t,e)=>Ge("get_artist",{artistId:t,fallbackName:e}),getAlbumTracks:t=>Ge("get_album_tracks",{albumId:t}),getPlaylistTracks:t=>Ge("get_playlist_tracks",{playlistId:t}),getRadioTracks:t=>Ge("get_radio_tracks",{trackId:t}),getLyrics:(t,e,r)=>Ge("get_lyrics",{title:t,artist:e,durationSecs:r??null}),resolveAudioStream:t=>Ge("resolve_audio_stream",{track:t}),prefetchAudio:t=>Ge("prefetch_audio",{track:t}),cancelStaleStreams:t=>Ge("cancel_stale_streams",{keepIds:t}),getCacheBytes:()=>Ge("get_cache_bytes"),clearAudioCache:()=>Ge("clear_audio_cache"),spinStatus:()=>Ge("spin_status"),spinStartLocal:()=>Ge("spin_start_local"),spinStopLocal:()=>Ge("spin_stop_local"),spinSetAllowControls:t=>Ge("spin_set_allow_controls",{allow:t}),spinStartRemoteHost:()=>Ge("spin_start_remote_host"),spinStopRemote:()=>Ge("spin_stop_remote"),spinJoinRemote:t=>Ge("spin_join_remote",{code:t}),spinSyncSnapshot:t=>Ge("spin_sync_snapshot",{isPlaying:t.isPlaying,positionSecs:t.positionSecs,sourceName:t.sourceName,currentIndex:t.currentIndex,queue:t.queue})};class ed{decks;activeIndex=0;volume=1;crossfadeSecs=0;crossfadeFired=!1;fadeRaf=null;loading=!1;callbacks;constructor(e){this.callbacks=e,this.decks=[this.createDeck(0),this.createDeck(1)]}createDeck(e){const r=document.createElement("audio");return r.preload="auto",r.addEventListener("timeupdate",()=>{if(e!==this.activeIndex)return;const a=Number.isFinite(r.currentTime)?r.currentTime:0,i=Number.isFinite(r.duration)?r.duration:0;this.callbacks.onTimeUpdate(a,i),this.crossfadeSecs>0&&!this.crossfadeFired&&i>this.crossfadeSecs+2&&i-a<=this.crossfadeSecs&&(this.crossfadeFired=!0,this.callbacks.onCrossfadeTrigger())}),r.addEventListener("play",()=>{e===this.activeIndex&&this.callbacks.onPlayStateChange(!0)}),r.addEventListener("pause",()=>{e===this.activeIndex&&this.callbacks.onPlayStateChange(!1)}),r.addEventListener("ended",()=>{e===this.activeIndex&&this.callbacks.onEnded()}),r.addEventListener("error",()=>{if(e===this.activeIndex&&r.src&&!this.loading){const a=r.error?.code,i=r.error?.message||(a?`Audio error (code ${a})`:"Audio playback error");console.error("[GyroTube audio error]",{src:r.src,code:a,message:r.error?.message,networkState:r.networkState,readyState:r.readyState}),this.callbacks.onError(i)}}),r}cancelFade(){this.fadeRaf!==null&&(cancelAnimationFrame(this.fadeRaf),this.fadeRaf=null)}get activeDeck(){return this.decks[this.activeIndex]}setVolume(e){this.volume=Math.max(0,Math.min(1,e)),this.activeDeck.volume=this.volume}setCrossfadeSecs(e){this.crossfadeSecs=Math.max(0,Math.min(10,Math.round(e)))}async loadAndPlay(e,r=0,a=!1){this.cancelFade(),this.crossfadeFired=!1,this.loading=!0;const i=this.activeIndex,o=a?i===0?1:0:i,s=this.decks[i],l=this.decks[o];if(this.activeIndex=o,l.src=e,l.load(),r>0){const v=r;l.addEventListener("loadedmetadata",()=>{l.src===e&&Number.isFinite(v)&&(l.currentTime=v)},{once:!0})}try{if(a&&this.crossfadeSecs>0&&i!==o){const v=s.volume;l.volume=.01,await l.play();const f=this.crossfadeSecs*1e3,x=performance.now(),g=k=>{const _=Math.min(1,(k-x)/f);s.volume=Math.max(0,v*(1-_)),l.volume=Math.min(1,this.volume*_),_<1&&this.activeIndex===o?this.fadeRaf=requestAnimationFrame(g):(this.fadeRaf=null,this.activeIndex!==i&&(s.pause(),s.removeAttribute("src"),s.load()))};this.fadeRaf=requestAnimationFrame(g)}else i!==o&&(s.pause(),s.removeAttribute("src"),s.load()),l.volume=this.volume,await l.play()}catch(v){throw console.error("[GyroTube play() failed]",{url:e,err:v,canPlayWebm:l.canPlayType("audio/webm"),canPlayMp4:l.canPlayType("audio/mp4"),errorCode:l.error?.code,errorMessage:l.error?.message}),v}finally{this.loading=!1}}async togglePlay(){const e=this.activeDeck;e.src&&(e.paused?await e.play():e.pause())}seek(e){const r=this.activeDeck;!r.src||!Number.isFinite(e)||(r.currentTime=Math.max(0,e))}stop(){this.cancelFade();for(const e of this.decks)e.pause(),e.removeAttribute("src"),e.load();this.callbacks.onPlayStateChange(!1),this.callbacks.onTimeUpdate(0,0)}updateMediaSession(e,r){if(!("mediaSession"in navigator))return;if(!e){navigator.mediaSession.metadata=null;return}const a=e.cover_large_url||e.cover_url;navigator.mediaSession.metadata=new MediaMetadata({title:e.title,artist:e.artist,album:e.album??"",artwork:a?[{src:a,sizes:"512x512",type:"image/jpeg"}]:[]}),navigator.mediaSession.setActionHandler("play",r.onPlay),navigator.mediaSession.setActionHandler("pause",r.onPause),navigator.mediaSession.setActionHandler("previoustrack",r.onPrev),navigator.mediaSession.setActionHandler("nexttrack",r.onNext),navigator.mediaSession.setActionHandler("seekto",i=>{i.seekTime!=null&&r.onSeek(i.seekTime)})}}async function Oa(t){return new Promise(e=>{const r=new Image;r.crossOrigin="anonymous",r.referrerPolicy="no-referrer",r.onload=()=>{try{const a=document.createElement("canvas");a.width=24,a.height=24;const i=a.getContext("2d");if(!i)return e(null);i.drawImage(r,0,0,24,24);const o=i.getImageData(0,0,24,24).data;let s=0,l=0,v=0,f=0;for(let x=0;x<o.length;x+=4){const g=o[x],k=o[x+1],_=o[x+2],S=Math.max(g,k,_),I=Math.min(g,k,_),$=S===0?0:(S-I)/S,y=(S+I)/510;if(y<.08||y>.92)continue;const b=.25+$*2.5;s+=g*b,l+=k*b,v+=_*b,f+=b}if(f===0)return e(null);e([Math.round(s/f),Math.round(l/f),Math.round(v/f)])}catch{e(null)}},r.onerror=()=>e(null),r.src=t})}function to(t,e=!1){return window.__TAURI_INTERNALS__.transformCallback(t,e)}async function Jn(t,e={},r){return window.__TAURI_INTERNALS__.invoke(t,e,r)}const td=Object.freeze(Object.defineProperty({__proto__:null,invoke:Jn,transformCallback:to},Symbol.toStringTag,{value:"Module"}));var Si;(function(t){t.WINDOW_RESIZED="tauri://resize",t.WINDOW_MOVED="tauri://move",t.WINDOW_CLOSE_REQUESTED="tauri://close-requested",t.WINDOW_DESTROYED="tauri://destroyed",t.WINDOW_FOCUS="tauri://focus",t.WINDOW_BLUR="tauri://blur",t.WINDOW_SCALE_FACTOR_CHANGED="tauri://scale-change",t.WINDOW_THEME_CHANGED="tauri://theme-changed",t.WINDOW_CREATED="tauri://window-created",t.WINDOW_SUSPENDED="tauri://suspended",t.WINDOW_RESUMED="tauri://resumed",t.WEBVIEW_CREATED="tauri://webview-created",t.DRAG_ENTER="tauri://drag-enter",t.DRAG_OVER="tauri://drag-over",t.DRAG_DROP="tauri://drag-drop",t.DRAG_LEAVE="tauri://drag-leave"})(Si||(Si={}));async function rd(t,e){window.__TAURI_EVENT_PLUGIN_INTERNALS__.unregisterListener(t,e),await Jn("plugin:event|unlisten",{event:t,eventId:e})}async function cn(t,e,r){var a;const i=(a=void 0)!==null&&a!==void 0?a:{kind:"Any"};return Jn("plugin:event|listen",{event:t,target:i,handler:to(e)}).then(o=>async()=>rd(t,o))}class ad{#r=G("home");get activeTab(){return n(this.#r)}set activeTab(e){z(this.#r,e,!0)}#l=G(Je({kind:"home"}));get activeView(){return n(this.#l)}set activeView(e){z(this.#l,e,!0)}#e=G(!0);get sidebarOpen(){return n(this.#e)}set sidebarOpen(e){z(this.#e,e,!0)}#s=G(!1);get navCollapsed(){return n(this.#s)}set navCollapsed(e){z(this.#s,e,!0)}#a=G("player");get sidebarTab(){return n(this.#a)}set sidebarTab(e){z(this.#a,e,!0)}#t=G(!1);get spinOpen(){return n(this.#t)}set spinOpen(e){z(this.#t,e,!0)}#i=G(!1);get partyMode(){return n(this.#i)}set partyMode(e){z(this.#i,e,!0)}#n=G(!1);get settingsOpen(){return n(this.#n)}set settingsOpen(e){z(this.#n,e,!0)}#o=G(Je([]));get queue(){return n(this.#o)}set queue(e){z(this.#o,e,!0)}#f=G(Je([]));get unshuffledQueue(){return n(this.#f)}set unshuffledQueue(e){z(this.#f,e,!0)}#c=G(null);get currentIndex(){return n(this.#c)}set currentIndex(e){z(this.#c,e,!0)}#h=G("Not playing");get sourceName(){return n(this.#h)}set sourceName(e){z(this.#h,e,!0)}#g=G(!1);get isPlaying(){return n(this.#g)}set isPlaying(e){z(this.#g,e,!0)}#d=G(!1);get isLoadingAudio(){return n(this.#d)}set isLoadingAudio(e){z(this.#d,e,!0)}#p=G(0);get positionSecs(){return n(this.#p)}set positionSecs(e){z(this.#p,e,!0)}#x=G(0);get durationSecs(){return n(this.#x)}set durationSecs(e){z(this.#x,e,!0)}#w=G("YTM");get qualityLabel(){return n(this.#w)}set qualityLabel(e){z(this.#w,e,!0)}#b=G(1);get volume(){return n(this.#b)}set volume(e){z(this.#b,e,!0)}#u=G(!1);get shuffle(){return n(this.#u)}set shuffle(e){z(this.#u,e,!0)}#_=G(!1);get repeat(){return n(this.#_)}set repeat(e){z(this.#_,e,!0)}#m=G(0);get crossfadeSecs(){return n(this.#m)}set crossfadeSecs(e){z(this.#m,e,!0)}#v=G(!0);get autoplayRelated(){return n(this.#v)}set autoplayRelated(e){z(this.#v,e,!0)}#y=G(!0);get backgroundPlayback(){return n(this.#y)}set backgroundPlayback(e){z(this.#y,e,!0)}#k=G("system");get themeMode(){return n(this.#k)}set themeMode(e){z(this.#k,e,!0)}#S=G("Auto");get language(){return n(this.#S)}set language(e){z(this.#S,e,!0)}#$=G(null);get ambientRgb(){return n(this.#$)}set ambientRgb(e){z(this.#$,e,!0)}#P=G(Je({type:"None"}));get lyrics(){return n(this.#P)}set lyrics(e){z(this.#P,e,!0)}#T=G(null);get activeLyricIndex(){return n(this.#T)}set activeLyricIndex(e){z(this.#T,e,!0)}#A=G(Je([]));get suggestedTracks(){return n(this.#A)}set suggestedTracks(e){z(this.#A,e,!0)}#z=G(Je([]));get favorites(){return n(this.#z)}set favorites(e){z(this.#z,e,!0)}#N=G(Je([]));get recentTracks(){return n(this.#N)}set recentTracks(e){z(this.#N,e,!0)}#M=G(Je([]));get userPlaylists(){return n(this.#M)}set userPlaylists(e){z(this.#M,e,!0)}#E=G(Je([]));get searchHistory(){return n(this.#E)}set searchHistory(e){z(this.#E,e,!0)}#C=G(null);get homeFeed(){return n(this.#C)}set homeFeed(e){z(this.#C,e,!0)}#I=G(!1);get homeLoading(){return n(this.#I)}set homeLoading(e){z(this.#I,e,!0)}#L=G(Je([]));get genres(){return n(this.#L)}set genres(e){z(this.#L,e,!0)}#R=G("");get searchQuery(){return n(this.#R)}set searchQuery(e){z(this.#R,e,!0)}#j=G(null);get searchResults(){return n(this.#j)}set searchResults(e){z(this.#j,e,!0)}#q=G(!1);get searchLoading(){return n(this.#q)}set searchLoading(e){z(this.#q,e,!0)}#O=G(Je({local_active:!1,local_url:"",local_qr_svg:null,allow_controls:!1,guest_count:0,added_count:0,remote_role:"idle",remote_ticket:null,remote_qr_svg:null}));get spinStatus(){return n(this.#O)}set spinStatus(e){z(this.#O,e,!0)}#D=G(Je([]));get toasts(){return n(this.#D)}set toasts(e){z(this.#D,e,!0)}toastSeq=0;loadGen=0;engine;#F=J(()=>this.currentIndex!=null&&this.currentIndex>=0&&this.currentIndex<this.queue.length?this.queue[this.currentIndex]:null);get currentTrack(){return n(this.#F)}set currentTrack(e){z(this.#F,e)}#H=J(()=>this.currentIndex!=null&&this.currentIndex+1<this.queue.length?this.queue[this.currentIndex+1]:null);get nextTrack(){return n(this.#H)}set nextTrack(e){z(this.#H,e)}constructor(){this.engine=new ed({onTimeUpdate:(e,r)=>{this.positionSecs=e,r>0&&(this.durationSecs=r),this.updateActiveLyric(e)},onPlayStateChange:e=>{this.isPlaying=e,e&&(this.isLoadingAudio=!1),this.syncSpinSnapshot()},onEnded:()=>{this.handleTrackEnded()},onCrossfadeTrigger:()=>{this.next(!0)},onError:e=>{this.isLoadingAudio=!1,this.showToast(e)}})}async bootstrap(){if(this.applyTheme(this.themeMode),!!Ue()){try{const e=await Ne.loadInitialState();this.favorites=e.favorites,this.recentTracks=e.recent_tracks,this.userPlaylists=e.user_playlists,this.searchHistory=e.search_history;const r=e.session;this.sidebarOpen=!r.sidebar_collapsed,this.queue=r.queue,this.unshuffledQueue=r.unshuffled_queue,this.currentIndex=r.current_index,this.sourceName=r.source_name||"Not playing",this.volume=r.user_volume??1,this.shuffle=r.shuffle,this.repeat=r.repeat,this.crossfadeSecs=r.crossfade_secs??0,this.autoplayRelated=r.autoplay_related??!0,this.backgroundPlayback=r.background_playback??!0,this.language=r.language??"Auto",this.themeMode=r.theme_mode||"system",typeof window<"u"&&window.innerWidth<1024&&(this.sidebarOpen=!1),this.engine.setVolume(this.volume),this.engine.setCrossfadeSecs(this.crossfadeSecs),this.applyTheme(this.themeMode);const a=this.currentTrack;if(a){this.durationSecs=a.duration_secs??0;const i=a.cover_large_url||a.cover_url;i&&Oa(i).then(o=>{this.currentTrack?.id===a.id&&(this.ambientRgb=o)}),Ne.getLyrics(a.title,a.artist,a.duration_secs).then(o=>{this.currentTrack?.id===a.id&&(this.lyrics=o)})}}catch(e){this.showToast(String(e))}this.loadHome(),this.loadGenres(),this.refreshSpinStatus(),this.bindNativeEvents()}}async bindNativeEvents(){if(Ue())try{await cn("tray:action",e=>{e.payload==="toggle_play"?this.togglePlay():e.payload==="next"?this.next():e.payload==="prev"&&this.prev()}),await cn("spin:command",e=>{const r=e.payload;r.kind==="toggle_play_pause"?this.togglePlay():r.kind==="next_track"?this.next():r.kind==="prev_track"?this.prev():r.kind==="move_track"?this.moveQueueItem(r.from,r.to):r.kind==="add_track"&&Ne.searchAll(r.video_id).then(a=>{const i=a.tracks.find(o=>o.id===r.video_id)??a.tracks[0];i&&(this.enqueueTrack(i),this.showToast(`${r.guest_name} added ${i.title}`))})}),await cn("spin:remote-sync",e=>{const{payload:r,error:a}=e.payload;if(a){this.showToast(a);return}if(!r?.track)return;const i=Math.max(0,Date.now()-r.sent_at_ms),o=r.position_secs+(r.is_playing?i/1e3:0);this.currentTrack?.id!==r.track.id?(this.queue=[r.track],this.unshuffledQueue=[r.track],this.sourceName=`SPIN · ${r.host_name||"Host"}`,this.playIndex(0,!1,o)):(Math.abs(this.positionSecs-o)>1.8&&this.seek(o),r.is_playing!==this.isPlaying&&this.engine.togglePlay())})}catch{}}applyTheme(e){if(this.themeMode=e,typeof document>"u")return;const r=document.documentElement;e==="system"?r.removeAttribute("data-theme"):r.setAttribute("data-theme",e)}setThemeMode(e){this.applyTheme(e),this.persistSession()}setLanguage(e){this.language=e,this.persistSession()}setCrossfadeSecs(e){this.crossfadeSecs=Math.max(0,Math.min(10,Math.round(e))),this.engine.setCrossfadeSecs(this.crossfadeSecs),this.persistSession()}setAutoplay(e){this.autoplayRelated=e,this.persistSession()}setBackgroundPlayback(e){this.backgroundPlayback=e,this.persistSession()}setVolume(e){this.volume=Math.max(0,Math.min(1,e)),this.engine.setVolume(this.volume),this.persistSession()}showToast(e){const r=++this.toastSeq;this.toasts=[...this.toasts,{id:r,message:e}],window.setTimeout(()=>{this.toasts=this.toasts.filter(a=>a.id!==r)},3200)}navigateTab(e){e==="explore"&&this.activeTab==="explore"&&this.activeView.kind==="explore"&&this.clearSearch(),this.activeTab=e,this.activeView={kind:e}}clearSearch(){this.searchQuery="",this.searchResults=null,this.searchLoading=!1}openArtist(e,r){this.activeTab="explore",this.activeView={kind:"artist",artistId:e,artistName:r}}openAlbum(e,r,a,i=!1){this.activeView={kind:"album",id:e,title:r,coverUrl:a,isPlaylist:i}}async loadHome(e=!1){if(Ue()){this.homeLoading=!0;try{this.homeFeed=await Ne.getHomeFeed(e)}catch(r){this.showToast(String(r))}finally{this.homeLoading=!1}}}async loadGenres(e=!1){if(Ue())try{this.genres=await Ne.getGenres(e)}catch{}}async runSearch(e){const r=e.trim();if(r&&(this.searchQuery=r,this.activeTab="explore",this.activeView={kind:"explore"},this.searchHistory=[r,...this.searchHistory.filter(a=>a.toLowerCase()!==r.toLowerCase())].slice(0,8),Ue())){Ne.saveSearchHistory(this.searchHistory),this.searchLoading=!0;try{this.searchResults=await Ne.searchAll(r)}catch(a){this.showToast(String(a))}finally{this.searchLoading=!1}}}clearSearchHistory(){this.searchHistory=[],Ue()&&Ne.saveSearchHistory([])}removeSearchHistoryItem(e){this.searchHistory=this.searchHistory.filter(r=>r!==e),Ue()&&Ne.saveSearchHistory(this.searchHistory)}isFavorite(e){return this.favorites.some(r=>r.id===e)}toggleFavorite(e){this.isFavorite(e.id)?this.favorites=this.favorites.filter(r=>r.id!==e.id):this.favorites=[e,...this.favorites],Ue()&&Ne.saveFavorites(this.favorites)}replaceQueueAndPlay(e,r,a){e.length&&(this.queue=[...e],this.unshuffledQueue=[...e],this.sourceName=a,this.shuffle=!1,this.playIndex(Math.max(0,Math.min(r,e.length-1))))}enqueueTrack(e){this.queue=[...this.queue,e],this.showToast(`Added to queue: ${e.title}`),this.currentIndex==null?this.playIndex(0):this.persistSession()}async startRadioFromCurrent(){const e=this.currentTrack;if(!(!e||!Ue()))try{const a=(await Ne.getRadioTracks(e.id)).filter(i=>i.id!==e.id);if(a.length===0){this.showToast("No related radio tracks found");return}this.queue=[e,...a],this.unshuffledQueue=[...this.queue],this.currentIndex=0,this.shuffle=!1,this.sourceName=`Radio · ${e.title}`,this.persistSession(),this.syncSpinSnapshot(),this.showToast(`Radio started · ${a.length} songs queued`)}catch(r){this.showToast(String(r))}}removeQueueIndex(e){if(e<0||e>=this.queue.length)return;const r=this.queue.filter((a,i)=>i!==e);if(this.queue=r,this.currentIndex!=null){if(r.length===0)this.currentIndex=null,this.engine.stop();else if(e===this.currentIndex){const a=Math.min(e,r.length-1);this.playIndex(a)}else e<this.currentIndex&&(this.currentIndex-=1);this.persistSession()}}moveQueueItem(e,r){if(e===r||e<0||r<0||e>=this.queue.length||r>=this.queue.length)return;const a=[...this.queue],[i]=a.splice(e,1);a.splice(r,0,i),this.queue=a,this.currentIndex===e?this.currentIndex=r:this.currentIndex!=null&&(e<this.currentIndex&&r>=this.currentIndex?this.currentIndex-=1:e>this.currentIndex&&r<=this.currentIndex&&(this.currentIndex+=1)),this.persistSession()}clearQueue(){const e=this.currentTrack;if(e){this.queue=[e],this.unshuffledQueue=[e],this.currentIndex=0,this.shuffle=!1,this.persistSession(),this.syncSpinSnapshot(),this.showToast("Queue cleared");return}this.queue=[],this.unshuffledQueue=[],this.currentIndex=null,this.engine.stop(),this.lyrics={type:"None"},this.ambientRgb=null,this.persistSession()}async playIndex(e,r=!1,a=0){if(e<0||e>=this.queue.length)return;const i=this.queue[e],o=++this.loadGen;this.currentIndex=e,this.isLoadingAudio=!0,this.positionSecs=0,this.durationSecs=i.duration_secs??0,this.lyrics={type:"None"},this.activeLyricIndex=null,this.recentTracks=[i,...this.recentTracks.filter(l=>l.id!==i.id)].slice(0,30),Ue()&&Ne.saveRecentTracks(this.recentTracks);const s=i.cover_large_url||i.cover_url;if(s?Oa(s).then(l=>{this.loadGen===o&&(this.ambientRgb=l)}):this.ambientRgb=null,this.engine.updateMediaSession(i,{onPlay:()=>void this.togglePlay(),onPause:()=>void this.togglePlay(),onPrev:()=>void this.prev(),onNext:()=>void this.next(),onSeek:l=>this.seek(l)}),this.persistSession(),!Ue()){this.isLoadingAudio=!1;return}try{Ne.cancelStaleStreams([i.id]);const l=await Ne.resolveAudioStream(i);if(this.loadGen!==o)return;this.qualityLabel=l.quality_label,await this.engine.loadAndPlay(l.url,a,r),this.syncSpinSnapshot(),Ne.getLyrics(i.title,i.artist,i.duration_secs).then(v=>{this.loadGen===o&&(this.lyrics=v)}),Ne.getRadioTracks(i.id).then(v=>{if(this.loadGen!==o)return;const f=new Set(this.queue.map(g=>g.id)),x=v.filter(g=>!f.has(g.id));if(this.autoplayRelated&&this.queue.length===1&&x.length>0){const g=x.slice(0,12);this.queue=[...this.queue,...g],this.unshuffledQueue=[...this.queue],this.suggestedTracks=x.slice(12,24),this.persistSession(),this.syncSpinSnapshot()}else this.suggestedTracks=x.slice(0,12)})}catch(l){this.loadGen===o&&(this.isLoadingAudio=!1,this.showToast(String(l)))}}async togglePlay(){if(!(!this.currentTrack||this.currentIndex==null)){if(!this.engine.activeDeck.src){await this.playIndex(this.currentIndex);return}await this.engine.togglePlay()}}seek(e){this.positionSecs=e,this.engine.seek(e),this.updateActiveLyric(e)}async prev(){if(this.positionSecs>3){this.seek(0);return}this.currentIndex!=null&&this.currentIndex>0&&await this.playIndex(this.currentIndex-1)}async next(e=!1){if(this.currentIndex!=null){if(this.currentIndex+1<this.queue.length)await this.playIndex(this.currentIndex+1,e);else if(this.repeat&&this.queue.length>0)await this.playIndex(0,e);else if(this.autoplayRelated&&this.suggestedTracks.length>0){const r=this.suggestedTracks[0];this.suggestedTracks=this.suggestedTracks.slice(1),this.queue=[...this.queue,r],await this.playIndex(this.queue.length-1,e)}}}async handleTrackEnded(){this.currentIndex!=null&&(this.currentIndex+1<this.queue.length||this.repeat||this.autoplayRelated)&&await this.next(!1)}toggleShuffle(){if(this.shuffle=!this.shuffle,this.queue.length<=1||this.currentIndex==null)return;const e=this.queue[this.currentIndex];if(this.shuffle){this.unshuffledQueue=[...this.queue];const r=this.queue.filter((a,i)=>i!==this.currentIndex);for(let a=r.length-1;a>0;a--){const i=Math.floor(Math.random()*(a+1));[r[a],r[i]]=[r[i],r[a]]}this.queue=[e,...r],this.currentIndex=0}else if(this.unshuffledQueue.length>0){const r=this.unshuffledQueue.findIndex(a=>a.id===e.id);this.queue=[...this.unshuffledQueue],this.currentIndex=r>=0?r:0}this.persistSession()}toggleRepeat(){this.repeat=!this.repeat,this.persistSession()}updateActiveLyric(e){if(this.lyrics.type!=="Synced")return;const r=this.lyrics.lines;let a=null;for(let i=r.length-1;i>=0;i--)if(r[i].time_secs<=e+.15){a=i;break}a!==this.activeLyricIndex&&(this.activeLyricIndex=a)}async refreshSpinStatus(){if(Ue())try{this.spinStatus=await Ne.spinStatus(),this.syncSpinSnapshot()}catch{}}async syncSpinSnapshot(){if(Ue()&&!(!this.spinStatus.local_active&&this.spinStatus.remote_role!=="host"))try{await Ne.spinSyncSnapshot({isPlaying:this.isPlaying,positionSecs:this.positionSecs,sourceName:this.sourceName,currentIndex:this.currentIndex,queue:this.queue})}catch{}}createUserPlaylist(e,r="",a="#2190a4",i=null){const o=e.trim()||"New playlist",s={id:`pl-${Date.now()}-${Math.random().toString(36).slice(2,7)}`,name:o,description:r.trim(),color_hex:a,icon_name:i,cover_path:null,tracks:[]};return this.userPlaylists=[s,...this.userPlaylists],Ue()&&Ne.saveUserPlaylists(this.userPlaylists),s}updateUserPlaylist(e,r,a,i,o){const s=r.trim();s&&(this.userPlaylists=this.userPlaylists.map(l=>l.id===e?{...l,name:s,description:a.trim(),color_hex:i,icon_name:o}:l),Ue()&&Ne.saveUserPlaylists(this.userPlaylists))}deleteUserPlaylist(e){this.userPlaylists=this.userPlaylists.filter(r=>r.id!==e),Ue()&&Ne.saveUserPlaylists(this.userPlaylists)}addTrackToPlaylist(e,r){let a=!1;this.userPlaylists=this.userPlaylists.map(i=>i.id!==e||i.tracks.some(o=>o.id===r.id)?i:(a=!0,{...i,tracks:[...i.tracks,r]})),a&&(Ue()&&Ne.saveUserPlaylists(this.userPlaylists),this.showToast(`Added "${r.title}" to playlist`))}removeTrackFromPlaylist(e,r){this.userPlaylists=this.userPlaylists.map(a=>a.id===e?{...a,tracks:a.tracks.filter(i=>i.id!==r)}:a),Ue()&&Ne.saveUserPlaylists(this.userPlaylists)}async persistSession(){if(!Ue())return;const e={sidebar_collapsed:!this.sidebarOpen,queue:this.queue,unshuffled_queue:this.unshuffledQueue,current_index:this.currentIndex,source_name:this.sourceName,user_volume:this.volume,shuffle:this.shuffle,repeat:this.repeat,position_secs:this.positionSecs,crossfade_secs:this.crossfadeSecs,language:this.language,background_playback:this.backgroundPlayback,autoplay_related:this.autoplayRelated,theme_mode:this.themeMode};try{await Ne.saveSession(e)}catch{}}}const c=new ad,nd={"brand.music":"Music","nav.home":"Home","nav.explore":"Explore","nav.collection":"Collection","nav.back":"Back","header.toggle_sidebar":"Toggle player sidebar","header.go_home":"Go to Home","header.search_placeholder":"Search songs, albums, artists…","header.clear_search":"Clear search","header.recent_searches":"Recent searches","header.suggestions":"Suggestions","header.clear":"Clear","header.spin_mode":"SPIN Mode","header.party_mode":"Party Mode","header.settings":"Settings","player.playing_from":"Playing from","player.not_playing":"Not playing","player.nothing_playing":"Nothing playing","player.select_song":"Select a song","player.album_prefix":"Album: {album}","player.collapse":"Collapse player","player.volume":"Volume","player.mute":"Mute","player.unmute":"Unmute","player.favorite_add":"Add to favorites","player.favorite_remove":"Remove from favorites","player.start_radio":"Start radio from this song","player.sleep_timer":"Sleep timer","player.sleep_off":"Off","player.shuffle":"Shuffle","player.previous":"Previous","player.play_pause":"Play / Pause","player.next":"Next","player.repeat":"Repeat","player.tab_player":"Player","player.tab_lyrics":"Lyrics","player.tab_queue":"Queue","lyrics.title":"Lyrics","lyrics.synced":"Synced","lyrics.static":"Static","lyrics.sync_btn":"Sync","lyrics.none_track":"No lyrics available for this track.","lyrics.none_idle":"Play a song to view its lyrics.","queue.title":"Queue","queue.clear":"Clear","queue.empty":"The queue is empty.","queue.remove":"Remove from queue","queue.autoplay":"Autoplay","queue.autoplay_desc":"Queue similar songs when queue ends","queue.suggestions":"Up next suggestions","queue.add":"Add to queue","home.morning":"Good morning","home.afternoon":"Good afternoon","home.evening":"Good evening","home.subtitle":"Your music ready to play","home.reload":"Reload home","home.new_releases":"New releases","home.new_releases_sub":"Recent albums & singles","home.top_charts":"Top charts","home.top_charts_sub":"Trending & Top Music","home.moods":"Moods","home.moods_sub":"Explore genres & vibes","home.collection":"Collection","home.collection_sub":"Your favorite songs","home.listen_again":"Listen again","home.featured":"FEATURED","home.play":"Play","home.open_album":"Open album","home.trending_hits":"Trending hits","home.tab_trending":"Trending","home.tab_top":"Top Music","home.stations_title":"Stations & genres","home.see_all":"See all →","home.station_badge":"Station","home.station_sub":"Curated playlists & mixes for this style.","home.explore_station":"Explore station","explore.title":"Explore","explore.subtitle":"Moods, moments, and music genres from YouTube Music","explore.moods":"Moods & moments","explore.genres":"Music genres","explore.back":"← Explore","explore.all":"All","explore.songs":"Songs","explore.albums":"Albums","explore.artists":"Artists","explore.playlists":"Playlists","explore.top_result":"Top result","explore.artist_pill":"Artist","collection.title":"Collection","collection.subtitle":"Your local playlists, recent history, and favorite songs","collection.new_playlist":"New playlist","collection.local_playlists":"Local playlists","collection.favorites":"Favorites","collection.favorites_desc":"Songs you marked with a heart","collection.recently_played":"Recently played","collection.play_all":"Play all","collection.empty_recent":"Songs you play will appear here.","collection.empty_favorites":"Tap the heart on any song to save it here.","collection.empty_favorites_title":"No favorites yet","collection.empty_playlist":"Add songs from Recently played or search results.","collection.empty_title":"This playlist is empty","collection.explore_music":"Explore music","collection.add_to_playlist":"Add to playlist:","collection.remove_from_playlist":"Remove from playlist","collection.customize":"Customize","collection.delete":"Delete","collection.sort":"Sort: {mode}","collection.sort_custom":"Custom","collection.sort_title":"Title","collection.sort_artist":"Artist","collection.sort_duration":"Duration","collection.modal_new":"New playlist","collection.modal_edit":"Customize playlist","collection.modal_name":"Name","collection.modal_name_ph":"Late Night Drive, Chill Mix…","collection.modal_desc":"Description","collection.modal_desc_ph":"Optional subtitle or vibe","collection.modal_color":"Accent color","collection.modal_auto":"Auto","collection.modal_icon":"Icon","collection.modal_cancel":"Cancel","collection.modal_create":"Create playlist","collection.modal_save":"Save changes","collection.untitled":"Untitled playlist","collection.default_desc":"Your curated local collection","collection.local_badge":"Local playlist","collection.col_title":"Title","collection.col_album":"Album","collection.song_one":"1 song","collection.song_many":"{count} songs","artist.verified":"Verified Artist","artist.play":"Play","artist.shuffle":"Shuffle","artist.mix":"Mix","artist.popular":"Popular songs","artist.show_less":"Show less","artist.more":"More","artist.show_all":"Show all ({count})","artist.albums":"Albums","artist.playlists":"Playlists","artist.similar":"Fans might also like","artist.about":"About {name}","album.badge_album":"Album","album.badge_playlist":"Playlist","album.no_tracks":"No tracks found.","party.badge":"Party Mode","party.exit":"Exit","party.up_next":"Up Next","settings.title":"Settings","settings.appearance":"Appearance","settings.theme":"Theme","settings.theme_sub":"Choose interface color scheme","settings.theme_system":"System","settings.theme_dark":"Dark","settings.theme_light":"Light","settings.language":"Language","settings.language_sub":"Interface display language","settings.lang_auto":"Auto","settings.playback":"Playback","settings.crossfade":"Crossfade","settings.crossfade_off":"Disabled (0 s)","settings.crossfade_secs":"{secs} seconds","settings.autoplay":"Autoplay","settings.autoplay_sub":"Queue similar songs when the queue ends","settings.background":"Background playback","settings.background_sub":"Keep playing in the system tray when closing the window","settings.storage":"Storage","settings.cache":"Audio cache","settings.cache_sub":"{mb} MB / 1024 MB cached","settings.search_history":"Search history","settings.search_history_one":"1 saved search","settings.search_history_many":"{count} saved searches","settings.clear":"Clear","spin.title":"SPIN Mode","spin.live":"Live","spin.close":"Close SPIN mode","spin.tab_local":"Wi-Fi Jukebox","spin.tab_remote":"App to App","spin.local_enable":"Enable local Jukebox","spin.local_enable_sub":"Local web server on your Wi-Fi network","spin.allow_controls":"Allow controls","spin.allow_controls_sub":"Pause, skip, and reorder the queue","spin.status":"Status","spin.status_active":"{guests} guests · {added} added","spin.status_inactive":"Inactive","spin.scan_to_join":"Scan to join","spin.link_copied":"SPIN link copied","spin.host_enable":"Broadcast as host","spin.host_enable_sub":"Create a synced listening invitation","spin.active_invite":"Active invitation","spin.invite_copied":"Invitation copied","spin.join_title":"Join a room","spin.join_sub":"Paste a SPIN invitation to listen in sync.","spin.join_placeholder":"GYRO-… or host URL","spin.join_btn":"Join","spin.disconnect_btn":"Disconnect","spin.connecting":"Connecting to SPIN room…","spin.copy":"Copy","spin.copied":"Copied","spin.copy_failed":"Could not copy to clipboard"},id={"brand.music":"Música","nav.home":"Inicio","nav.explore":"Explorar","nav.collection":"Colección","nav.back":"Volver","header.toggle_sidebar":"Mostrar/ocultar reproductor","header.go_home":"Ir a Inicio","header.search_placeholder":"Buscar canciones, álbumes, artistas…","header.clear_search":"Limpiar búsqueda","header.recent_searches":"Búsquedas recientes","header.suggestions":"Sugerencias","header.clear":"Limpiar","header.spin_mode":"Modo SPIN","header.party_mode":"Modo fiesta","header.settings":"Ajustes","player.playing_from":"Reproduciendo desde","player.not_playing":"Sin reproducción","player.nothing_playing":"Nada sonando","player.select_song":"Elige una canción","player.album_prefix":"Álbum: {album}","player.collapse":"Ocultar reproductor","player.volume":"Volumen","player.mute":"Silenciar","player.unmute":"Activar sonido","player.favorite_add":"Agregar a favoritos","player.favorite_remove":"Quitar de favoritos","player.start_radio":"Iniciar radio desde esta canción","player.sleep_timer":"Temporizador","player.sleep_off":"No","player.shuffle":"Aleatorio","player.previous":"Anterior","player.play_pause":"Reproducir / Pausar","player.next":"Siguiente","player.repeat":"Repetir","player.tab_player":"Reproductor","player.tab_lyrics":"Letras","player.tab_queue":"Cola","lyrics.title":"Letras","lyrics.synced":"Sincronizada","lyrics.static":"Estática","lyrics.sync_btn":"Sincronizar","lyrics.none_track":"No hay letras disponibles para esta canción.","lyrics.none_idle":"Reproduce una canción para ver su letra.","queue.title":"Cola","queue.clear":"Limpiar","queue.empty":"La cola está vacía.","queue.remove":"Quitar de la cola","queue.autoplay":"Reproducción automática","queue.autoplay_desc":"Agrega canciones similares al terminar la cola","queue.suggestions":"Sugerencias a continuación","queue.add":"Añadir a la cola","home.morning":"Buenos días","home.afternoon":"Buenas tardes","home.evening":"Buenas noches","home.subtitle":"Tu música lista para sonar","home.reload":"Recargar inicio","home.new_releases":"Nuevos lanzamientos","home.new_releases_sub":"Álbumes y sencillos recientes","home.top_charts":"Listas de éxitos","home.top_charts_sub":"Tendencias y Top Música","home.moods":"Estados de ánimo","home.moods_sub":"Explora géneros y estilos","home.collection":"Colección","home.collection_sub":"Tus canciones favoritas","home.listen_again":"Escuchar de nuevo","home.featured":"DESTACADO","home.play":"Reproducir","home.open_album":"Abrir álbum","home.trending_hits":"Éxitos en tendencia","home.tab_trending":"Tendencias","home.tab_top":"Top Música","home.stations_title":"Estaciones y géneros","home.see_all":"Ver todo →","home.station_badge":"Estación","home.station_sub":"Playlists y mezclas seleccionadas para este estilo.","home.explore_station":"Explorar estación","explore.title":"Explorar","explore.subtitle":"Estados de ánimo, momentos y géneros de YouTube Music","explore.moods":"Estados de ánimo y momentos","explore.genres":"Géneros musicales","explore.back":"← Explorar","explore.all":"Todo","explore.songs":"Canciones","explore.albums":"Álbumes","explore.artists":"Artistas","explore.playlists":"Playlists","explore.top_result":"Mejor resultado","explore.artist_pill":"Artista","collection.title":"Colección","collection.subtitle":"Tus playlists locales, historial reciente y canciones favoritas","collection.new_playlist":"Nueva playlist","collection.local_playlists":"Playlists locales","collection.favorites":"Favoritos","collection.favorites_desc":"Canciones que marcaste con corazón","collection.recently_played":"Escuchado recientemente","collection.play_all":"Reproducir todo","collection.empty_recent":"Las canciones que escuches aparecerán aquí.","collection.empty_favorites":"Toca el corazón en cualquier canción para guardarla aquí.","collection.empty_favorites_title":"Sin favoritos aún","collection.empty_playlist":"Agrega canciones desde el historial reciente o resultados de búsqueda.","collection.empty_title":"Esta playlist está vacía","collection.explore_music":"Explorar música","collection.add_to_playlist":"Agregar a playlist:","collection.remove_from_playlist":"Quitar de la playlist","collection.customize":"Personalizar","collection.delete":"Eliminar","collection.sort":"Orden: {mode}","collection.sort_custom":"Personalizado","collection.sort_title":"Título","collection.sort_artist":"Artista","collection.sort_duration":"Duración","collection.modal_new":"Nueva playlist","collection.modal_edit":"Personalizar playlist","collection.modal_name":"Nombre","collection.modal_name_ph":"Viaje nocturno, Chill Mix…","collection.modal_desc":"Descripción","collection.modal_desc_ph":"Subtítulo o ambiente opcional","collection.modal_color":"Color de acento","collection.modal_auto":"Auto","collection.modal_icon":"Ícono","collection.modal_cancel":"Cancelar","collection.modal_create":"Crear playlist","collection.modal_save":"Guardar cambios","collection.untitled":"Playlist sin título","collection.default_desc":"Tu colección local curada","collection.local_badge":"Playlist local","collection.col_title":"Título","collection.col_album":"Álbum","collection.song_one":"1 canción","collection.song_many":"{count} canciones","artist.verified":"Artista verificado","artist.play":"Reproducir","artist.shuffle":"Aleatorio","artist.mix":"Mix","artist.popular":"Canciones populares","artist.show_less":"Mostrar menos","artist.more":"Más","artist.show_all":"Mostrar todas ({count})","artist.albums":"Álbumes","artist.playlists":"Playlists","artist.similar":"A los fans también les gusta","artist.about":"Acerca de {name}","album.badge_album":"Álbum","album.badge_playlist":"Playlist","album.no_tracks":"No se encontraron canciones.","party.badge":"Modo fiesta","party.exit":"Salir","party.up_next":"A continuación","settings.title":"Ajustes","settings.appearance":"Apariencia","settings.theme":"Tema","settings.theme_sub":"Elige el esquema de color de la interfaz","settings.theme_system":"Sistema","settings.theme_dark":"Oscuro","settings.theme_light":"Claro","settings.language":"Idioma","settings.language_sub":"Idioma de la interfaz","settings.lang_auto":"Auto","settings.playback":"Reproducción","settings.crossfade":"Crossfade","settings.crossfade_off":"Desactivado (0 s)","settings.crossfade_secs":"{secs} segundos","settings.autoplay":"Reproducción automática","settings.autoplay_sub":"Agrega canciones similares al terminar la cola","settings.background":"Reproducción en segundo plano","settings.background_sub":"Seguir reproduciendo en la bandeja al cerrar la ventana","settings.storage":"Almacenamiento","settings.cache":"Caché de audio","settings.cache_sub":"{mb} MB / 1024 MB en caché","settings.search_history":"Historial de búsqueda","settings.search_history_one":"1 búsqueda guardada","settings.search_history_many":"{count} búsquedas guardadas","settings.clear":"Limpiar","spin.title":"Modo SPIN","spin.live":"En vivo","spin.close":"Cerrar modo SPIN","spin.tab_local":"Jukebox Wi-Fi","spin.tab_remote":"App a App","spin.local_enable":"Activar Jukebox local","spin.local_enable_sub":"Servidor web local en tu red Wi-Fi","spin.allow_controls":"Permitir controles","spin.allow_controls_sub":"Pausar, saltar y reordenar la cola","spin.status":"Estado","spin.status_active":"{guests} invitados · {added} agregadas","spin.status_inactive":"Inactivo","spin.scan_to_join":"Escanea para unirte","spin.link_copied":"Enlace SPIN copiado","spin.host_enable":"Transmitir como anfitrión","spin.host_enable_sub":"Crea una invitación de escucha sincronizada","spin.active_invite":"Invitación activa","spin.invite_copied":"Invitación copiada","spin.join_title":"Unirse a una sala","spin.join_sub":"Pega una invitación SPIN para escuchar en sincronía.","spin.join_placeholder":"GYRO-… o URL del anfitrión","spin.join_btn":"Unirse","spin.disconnect_btn":"Desconectar","spin.connecting":"Conectando a la sala SPIN…","spin.copy":"Copiar","spin.copied":"Copiado","spin.copy_failed":"No se pudo copiar al portapapeles"},$i={en:nd,es:id};function sd(){return c.language==="Es"?"es":c.language==="En"?"en":typeof navigator<"u"&&(navigator.language||"en").toLowerCase().startsWith("es")?"es":"en"}function P(t,e){const r=sd();let a=$i[r][t]??$i.en[t]??t;if(e)for(const[i,o]of Object.entries(e))a=a.replaceAll(`{${i}}`,String(o));return a}var od=L('<div class="cover-skeleton svelte-1gzalhk" aria-hidden="true"></div>'),ld=L('<img decoding="async" referrerpolicy="no-referrer"/>'),cd=L("<!> <!>",1),dd=L('<div class="cover-fallback svelte-1gzalhk" aria-hidden="true"> </div>'),ud=L("<div><!></div>");function nt(t,e){Ae(e,!0);let r=et(e,"src",3,null),a=et(e,"fallbackSrc",3,null),i=et(e,"alt",3,""),o=et(e,"class",3,""),s=et(e,"fallbackIcon",3,"♪"),l=et(e,"eager",3,!1),v=G(null),f=G(!1),x=G(!1),g=G(0),k=G(!1),_=G(null);function S(){return e.targetPx?e.targetPx:o().includes("track-cover")||o().includes("tile-cover")||o().includes("up-next-cover")||o().includes("w-10")?120:o().includes("media-cover")||o().includes("artist-avatar")||o().includes("top-avatar")||o().includes("hero-cover")||o().includes("album-cover")?360:544}function I(N){if(!N)return null;const R=S();return N.includes("googleusercontent.com")&&/=w\d+-h\d+/.test(N)?N.replace(/=w\d+-h\d+[^&?]*/,`=w${R}-h${R}-l90-rj`):N.includes("i.ytimg.com")&&R<=120?N.replace(/\/(maxresdefault|sddefault|hqdefault)\./,"/mqdefault."):N}$t(()=>{const N=I(r())||I(a())||null;N!==n(_)&&(z(_,N,!0),z(g,0),z(k,!1),z(x,!1))}),Sa(()=>{if(l()){z(f,!0);return}if(!n(v)||typeof IntersectionObserver>"u"){z(f,!0);return}const N=new IntersectionObserver(R=>{R.some(p=>p.isIntersecting)&&(z(f,!0),N.disconnect())},{rootMargin:"160px"});return N.observe(n(v)),()=>N.disconnect()});function $(){if(n(g)===0&&a()){const N=I(a());if(N&&N!==n(_)){z(g,1),z(_,N,!0);return}}if(n(g)<2&&n(_)){if(z(g,2),n(_).includes("googleusercontent.com")&&n(_).includes("=w")){const N=n(_).replace(/=w\d+-h\d+[^&?]*/,"=w226-h226-l90-rj");if(N!==n(_)){z(_,N,!0);return}}if(n(_).includes("i.ytimg.com")&&!n(_).includes("hqdefault")){const N=n(_).replace(/\/(maxresdefault|sddefault|mqdefault)\./,"/hqdefault.");if(N!==n(_)){z(_,N,!0);return}}}z(k,!0)}var y=ud(),b=u(y);{var w=N=>{var R=cd(),p=K(R);{var j=q=>{var U=od();h(q,U)};Q(p,q=>{n(x)||q(j)})}var E=d(p,2);{var C=q=>{var U=ld();O(()=>{ee(U,1,`cover-img ${n(x)?"is-loaded":""}`,"svelte-1gzalhk"),le(U,"src",n(_)),le(U,"alt",i()),le(U,"loading",l()?"eager":"lazy")}),_t("load",U,()=>z(x,!0)),_t("error",U,$),h(q,U)};Q(E,q=>{(n(f)||l())&&q(C)})}h(N,R)},A=N=>{var R=dd(),p=T(R,!0);O(()=>m(p,s())),h(N,R)};Q(b,N=>{n(_)&&!n(k)?N(w):N(A,-1)})}Xr(y,N=>z(v,N),()=>n(v)),O(()=>ee(y,1,`cover-shell ${o()??""}`,"svelte-1gzalhk")),h(t,y),ze()}var vd=L('<span class="text-[13px] font-semibold text-[var(--text-muted)] truncate max-w-full"> </span>'),fd=L('<aside class="lg:sticky lg:top-8 flex flex-col items-center text-center gap-4"><div class="w-full flex items-center justify-start"><button class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-[12.5px] font-semibold text-[var(--text-muted)] hover:text-[var(--text)] bg-white/[0.05] hover:bg-white/[0.1] border border-white/10 transition-all active:scale-95" title="Back"><!> </button></div> <!> <!> <div class="space-y-1.5 max-w-[300px]"><h1 class="text-[24px] font-black tracking-[-0.025em] leading-tight"> </h1> <div class="flex items-center justify-center gap-2 text-[12.5px] font-medium text-[var(--text-muted)]"><span class="px-2 py-0.5 rounded-full text-[10.5px] font-bold uppercase tracking-wider bg-[var(--accent-soft)] text-[var(--accent-bright)]"> </span> <span>·</span> <span class="numeric"> </span></div></div> <div class="flex items-center justify-center gap-4 pt-2"><button class="w-11 h-11 rounded-full inline-flex items-center justify-center bg-white/[0.08] hover:bg-white/[0.15] border border-white/10 transition-all active:scale-95 disabled:opacity-40" title="Shuffle"><!></button> <button class="play-circle !w-[60px] !h-[60px]" title="Play all"><!></button> <button class="w-11 h-11 rounded-full inline-flex items-center justify-center bg-white/[0.08] hover:bg-white/[0.15] border border-white/10 transition-all active:scale-95 disabled:opacity-40" title="Add all to queue"><!></button></div></aside>');function hd(t,e){Ae(e,!0);let r=et(e,"coverUrl",3,null),a=et(e,"isPlaylist",3,!1),i=J(()=>Math.round(e.tracks.reduce((H,F)=>H+(F.duration_secs??0),0)/60)),o=J(()=>e.tracks[0]?.artist??"");function s(){e.tracks.length&&c.replaceQueueAndPlay(e.tracks,0,e.title)}function l(){if(!e.tracks.length)return;const H=[...e.tracks].sort(()=>Math.random()-.5);c.replaceQueueAndPlay(H,0,e.title)}function v(){if(e.tracks.length){for(const H of e.tracks)c.queue=[...c.queue,H];c.persistSession(),c.showToast(`Added ${e.tracks.length} songs to queue`)}}var f=fd(),x=u(f),g=u(x),k=u(g);Ga(k,{size:15});var _=d(k),S=d(x,2);{var I=H=>{var F=vd(),D=T(F,!0);O(()=>m(D,n(o))),h(H,F)};Q(S,H=>{n(o)&&H(I)})}var $=d(S,2);{let H=J(()=>e.tracks[0]?.cover_large_url??e.tracks[0]?.cover_url);nt($,{class:"album-cover !w-[248px] !h-[248px] !rounded-2xl border border-white/10 shadow-[0_22px_52px_rgba(0,0,0,0.52)]",get src(){return r()},get fallbackSrc(){return n(H)},get alt(){return e.title},eager:!0})}var y=d($,2),b=u(y),w=T(b,!0),A=d(b,2),N=u(A),R=T(N,!0),p=d(N,4),j=T(p),E=d(y,2),C=u(E),q=u(C);ta(q,{size:18});var U=d(C,2),X=u(U);bt(X,{size:24,fill:"currentColor",class:"translate-x-[1.5px]"});var B=d(U,2),W=u(B);Zr(W,{size:18}),O((H,F,D)=>{m(_,` ${H??""}`),m(w,e.title),m(R,F),m(j,`${D??""}${n(i)?` · ${n(i)} min`:""}`),C.disabled=e.tracks.length===0,U.disabled=e.tracks.length===0,B.disabled=e.tracks.length===0},[()=>P("nav.back"),()=>a()?P("album.badge_playlist"):P("album.badge_album"),()=>e.tracks.length===1?P("collection.song_one"):P("collection.song_many",{count:e.tracks.length})]),M("click",g,()=>c.navigateTab(c.activeTab)),M("click",C,l),M("click",U,s),M("click",B,v),h(t,f),ze()}Ve(["click"]);function zt(t){if(t==null||!Number.isFinite(t)||t<0)return"--:--";const e=Math.floor(t),r=Math.floor(e/60),a=e%60;return`${r}:${a.toString().padStart(2,"0")}`}function Da(t){return t?t>=1e6?`${(t/1e6).toFixed(1)} M subscribers`:t>=1e3?`${(t/1e3).toFixed(1)} K subscribers`:`${t} subscribers`:""}var pd=L('<div class="flex items-center gap-4 py-3.5 px-3"><div class="w-6 h-4 rounded bg-white/[0.07]"></div> <div class="flex-1 space-y-2"><div class="h-3.5 w-48 rounded bg-white/[0.09]"></div> <div class="h-3 w-28 rounded bg-white/[0.05]"></div></div> <div class="h-3 w-10 rounded bg-white/[0.06]"></div></div>'),_d=L('<div class="divide-y divide-white/[0.06] animate-pulse" aria-busy="true"></div>'),gd=L('<span class="text-[13px] font-semibold text-[var(--text-muted)] numeric group-hover:hidden"> </span> <!>',1),bd=L('<div role="button" tabindex="0"><div class="w-7 flex items-center justify-center shrink-0"><!></div> <!> <div class="min-w-0 flex-1"><div> </div> <div class="text-[12.5px] text-[var(--text-muted)] truncate"> </div></div> <div class="flex items-center gap-1.5 shrink-0"><button title="Favorite"><!></button> <button class="icon-btn !w-8 !h-8 opacity-65 group-hover:opacity-100" title="Add to queue"><!></button> <span class="text-[13px] text-[var(--text-muted)] numeric w-11 text-right"> </span></div></div>'),xd=L('<div class="flex items-center gap-4 px-3 pb-2.5 text-[11px] font-bold uppercase tracking-wider text-[var(--text-muted)]/75 border-b border-white/[0.08]"><span class="w-7 text-center">#</span> <span class="flex-1">Title</span> <span class="w-11 flex justify-end"><!></span></div> <div class="flex flex-col"></div>',1),md=L('<p class="text-[var(--text-muted)] py-10 text-center">No tracks found.</p>'),yd=L('<section class="min-w-0"><!></section>');function wd(t,e){Ae(e,!0);let r=et(e,"isPlaylist",3,!1);var a=yd(),i=u(a);{var o=v=>{var f=_d();Re(f,20,()=>[0,1,2,3,4,5,6,7],Rt,(x,g)=>{var k=pd();h(x,k)}),h(v,f)},s=v=>{var f=xd(),x=K(f),g=d(u(x),4),k=u(g);Ya(k,{size:13});var _=d(x,2);Re(_,23,()=>e.tracks,(S,I)=>S.id+":"+I,(S,I,$)=>{const y=J(()=>c.currentTrack?.id===n(I).id);var b=bd(),w=u(b),A=u(w);{var N=Y=>{vr(Y,{size:16,class:"text-[var(--accent-bright)]"})},R=Y=>{var pe=gd(),de=K(pe),oe=T(de,!0),re=d(de,2);bt(re,{size:14,fill:"currentColor",class:"hidden group-hover:block text-[var(--text)] translate-x-[1px]"}),O(()=>m(oe,n($)+1)),h(Y,pe)};Q(A,Y=>{n(y)?Y(N):Y(R,-1)})}var p=d(w,2);{var j=Y=>{nt(Y,{class:"track-cover !w-9 !h-9 !rounded-lg shrink-0",get src(){return n(I).cover_url},alt:""})};Q(p,Y=>{r()&&Y(j)})}var E=d(p,2),C=u(E),q=T(C,!0),U=d(C,2),X=T(U,!0),B=d(E,2),W=u(B),H=u(W);{let Y=J(()=>c.isFavorite(n(I).id)?"currentColor":"none");gt(H,{size:16,get fill(){return n(Y)}})}var F=d(W,2),D=u(F);yr(D,{size:16});var V=d(F,2),Z=T(V,!0);O((Y,pe)=>{ee(b,1,`group flex items-center gap-3.5 py-2.5 px-3 border-b border-white/[0.06] last:border-b-0 cursor-pointer transition-colors ${n(y)?"bg-[var(--accent-soft)]":"hover:bg-white/[0.05]"}`),ee(C,1,`text-[14.5px] font-semibold truncate ${n(y)?"text-[var(--accent-bright)] font-bold":""}`),m(q,n(I).title),m(X,n(I).artist),ee(W,1,`icon-btn !w-8 !h-8 ${Y??""}`),m(Z,pe)},[()=>c.isFavorite(n(I).id)?"fav-active":"opacity-65 group-hover:opacity-100",()=>zt(n(I).duration_secs)]),M("click",b,()=>c.replaceQueueAndPlay(e.tracks,n($),e.title)),M("keydown",b,Y=>Y.key==="Enter"&&c.replaceQueueAndPlay(e.tracks,n($),e.title)),M("click",W,Y=>{Y.stopPropagation(),c.toggleFavorite(n(I))}),M("click",F,Y=>{Y.stopPropagation(),c.enqueueTrack(n(I))}),h(S,b)}),h(v,f)},l=v=>{var f=md();h(v,f)};Q(i,v=>{e.loading?v(o):e.tracks.length>0?v(s,1):v(l,-1)})}h(t,a),ze()}Ve(["click","keydown"]);var kd=L('<div class="w-full min-h-full transition-all duration-500"><div class="w-full max-w-[1280px] mx-auto px-4 sm:px-8 pt-4 sm:pt-6 pb-36 md:pb-28"><div class="grid grid-cols-1 lg:grid-cols-[320px_minmax(0,1fr)] gap-8 lg:gap-12 items-start"><!> <!></div></div></div>');function Sd(t,e){Ae(e,!0);let r=et(e,"coverUrl",3,null),a=et(e,"isPlaylist",3,!1),i=G(Je([])),o=G(""),s=G(!1),l=G(null);$t(()=>{const S=e.id,I=a();if(z(o,e.title,!0),z(i,[],!0),z(l,null),r()&&Oa(r()).then(y=>{e.id===S&&z(l,y,!0)}),!Ue())return;z(s,!0),(I?Ne.getPlaylistTracks(S):Ne.getAlbumTracks(S)).then(([y,b])=>{e.id===S&&(y&&z(o,y,!0),z(i,b,!0),!r()&&b[0]?.cover_url&&Oa(b[0].cover_url).then(w=>{e.id===S&&z(l,w,!0)}))}).catch(y=>c.showToast(String(y))).finally(()=>{e.id===S&&z(s,!1)})});let v=J(()=>n(l)?`background-image: radial-gradient(ellipse 75% 48% at 24% 26%, rgba(${n(l)[0]}, ${n(l)[1]}, ${n(l)[2]}, 0.22) 0%, rgba(${n(l)[0]}, ${n(l)[1]}, ${n(l)[2]}, 0.07) 48%, transparent 78%);`:"");var f=kd(),x=u(f),g=u(x),k=u(g);hd(k,{get title(){return n(o)},get coverUrl(){return r()},get isPlaylist(){return a()},get tracks(){return n(i)}});var _=d(k,2);wd(_,{get title(){return n(o)},get isPlaylist(){return a()},get tracks(){return n(i)},get loading(){return n(s)}}),O(()=>Ke(f,n(v))),h(t,f),ze()}var $d=L('<span class="text-[12.5px] font-semibold text-[var(--text-muted)] numeric group-hover:hidden"> </span> <!>',1),Pd=L('<div role="button" tabindex="0"><div class="flex items-center justify-center shrink-0"><!></div> <div class="flex items-center gap-3.5 min-w-0"><!> <div class="min-w-0"><div> </div> <div class="text-[12px] text-[var(--text-muted)] truncate"> </div></div></div> <span class="hidden lg:block text-[13px] text-[var(--text-muted)] truncate"> </span> <div class="flex items-center justify-end gap-1 shrink-0 w-[108px]"><button type="button" class="w-8 h-8 rounded-full inline-flex items-center justify-center text-[var(--text-muted)] opacity-0 group-hover:opacity-100 focus-visible:opacity-100 hover:text-[var(--text)] hover:bg-white/[0.1] transition-all"><!></button> <button type="button"><!></button> <span class="text-[12.5px] text-[var(--text-muted)] numeric w-11 text-right"> </span></div></div>'),Td=L('<div class="w-full max-w-[1280px] mx-auto px-6 pt-6 pb-28 space-y-6"><div class="space-y-4"><button type="button" class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-[12.5px] font-semibold text-[var(--text-muted)] hover:text-[var(--text)] bg-white/[0.06] hover:bg-white/[0.12] border border-white/10 transition-all active:scale-95"><!> </button> <div class="flex flex-wrap items-end justify-between gap-4"><div class="space-y-1"><span class="text-[12px] font-bold uppercase tracking-[0.08em] text-[var(--accent-bright)]"> </span> <h1 class="text-[30px] sm:text-[36px] font-black tracking-[-0.03em] leading-tight"> </h1> <p class="text-[13px] text-[var(--text-muted)] numeric"> </p></div> <div class="flex items-center gap-2.5"><button type="button" class="pill-btn primary !px-5 !py-2"><!> </button> <button type="button" class="pill-btn !px-4 !py-2 bg-white/[0.07] hover:bg-white/[0.13] border-white/15"><!> </button></div></div></div> <div><div class="grid grid-cols-[28px_minmax(0,1fr)_auto] lg:grid-cols-[28px_minmax(0,1.65fr)_minmax(0,1fr)_auto] items-center gap-4 px-3 pb-2.5 text-[11px] font-bold uppercase tracking-wider text-[var(--text-muted)]/75 border-b border-white/[0.08]"><span class="text-center">#</span> <span> </span> <span class="hidden lg:block"> </span> <span class="w-[108px] flex justify-end"><!></span></div> <div class="flex flex-col"></div></div></div>');function Ad(t,e){Ae(e,!0);let r=J(()=>Math.round(e.tracks.reduce((D,V)=>D+(V.duration_secs??0),0)/60));function a(){e.tracks.length&&c.replaceQueueAndPlay(e.tracks,0,e.artistName)}function i(){if(!e.tracks.length)return;const D=[...e.tracks].sort(()=>Math.random()-.5);c.replaceQueueAndPlay(D,0,e.artistName)}var o=Td(),s=u(o),l=u(s),v=u(l);Ga(v,{size:15});var f=d(v),x=d(l,2),g=u(x),k=u(g),_=T(k,!0),S=d(k,2),I=T(S,!0),$=d(S,2),y=T($),b=d(g,2),w=u(b),A=u(w);bt(A,{size:15,fill:"currentColor"});var N=d(A),R=d(w,2),p=u(R);ta(p,{size:15});var j=d(p),E=d(s,2),C=u(E),q=d(u(C),2),U=T(q,!0),X=d(q,2),B=T(X,!0),W=d(X,2),H=u(W);Ya(H,{size:13});var F=d(C,2);Re(F,23,()=>e.tracks,(D,V)=>D.id+":"+V,(D,V,Z)=>{const Y=J(()=>c.currentTrack?.id===n(V).id),pe=J(()=>c.isFavorite(n(V).id));var de=Pd(),oe=u(de),re=u(oe);{var ae=ne=>{vr(ne,{size:15,class:"text-[var(--accent-bright)]"})},ge=ne=>{var Se=$d(),Ce=K(Se),$e=T(Ce,!0),Be=d(Ce,2);bt(Be,{size:13,fill:"currentColor",class:"hidden group-hover:block translate-x-[1px]"}),O(()=>m($e,n(Z)+1)),h(ne,Se)};Q(re,ne=>{n(Y)?ne(ae):ne(ge,-1)})}var Te=d(oe,2),ce=u(Te);nt(ce,{class:"track-cover !w-10 !h-10 !rounded-[8px] shrink-0",get src(){return n(V).cover_url},alt:""});var be=d(ce,2),me=u(be),Ee=T(me,!0),we=d(me,2),qe=T(we,!0),We=d(Te,2),Ye=T(We,!0),vt=d(We,2),lt=u(vt),ye=u(lt);Zr(ye,{size:16});var Pe=d(lt,2),ke=u(Pe);{let ne=J(()=>n(pe)?"currentColor":"none");gt(ke,{size:16,get fill(){return n(ne)}})}var Me=d(Pe,2),Oe=T(Me,!0);O((ne,Se,Ce)=>{ee(de,1,`group grid grid-cols-[28px_minmax(0,1fr)_auto] lg:grid-cols-[28px_minmax(0,1.65fr)_minmax(0,1fr)_auto] items-center gap-4 py-2.5 px-3 border-b border-white/[0.06] last:border-b-0 cursor-pointer transition-colors ${n(Y)?"bg-[var(--accent-soft)]":"hover:bg-white/[0.05]"}`),ee(me,1,`text-[14px] font-semibold truncate ${n(Y)?"text-[var(--accent-bright)] font-bold":""}`),m(Ee,n(V).title),m(qe,n(V).artist),m(Ye,n(V).album??""),le(lt,"title",ne),ee(Pe,1,`w-8 h-8 rounded-full inline-flex items-center justify-center transition-all ${n(pe)?"text-[var(--fav-coral)] opacity-100 hover:bg-white/[0.08]":"text-[var(--text-muted)] opacity-0 group-hover:opacity-100 focus-visible:opacity-100 hover:text-[var(--text)] hover:bg-white/[0.1]"}`),le(Pe,"title",Se),m(Oe,Ce)},[()=>P("queue.add"),()=>n(pe)?P("player.favorite_remove"):P("player.favorite_add"),()=>zt(n(V).duration_secs)]),M("click",de,()=>c.replaceQueueAndPlay(e.tracks,n(Z),e.artistName)),M("keydown",de,ne=>ne.key==="Enter"&&c.replaceQueueAndPlay(e.tracks,n(Z),e.artistName)),M("click",lt,ne=>{ne.stopPropagation(),c.enqueueTrack(n(V))}),M("click",Pe,ne=>{ne.stopPropagation(),c.toggleFavorite(n(V))}),h(D,de)}),O((D,V,Z,Y,pe,de)=>{m(f,` ${e.artistName??""}`),m(_,e.artistName),m(I,D),m(y,`${V??""}${n(r)?` · ${n(r)} min`:""}`),w.disabled=e.tracks.length===0,m(N,` ${Z??""}`),R.disabled=e.tracks.length===0,m(j,` ${Y??""}`),m(U,pe),m(B,de)},[()=>P("artist.popular"),()=>e.tracks.length===1?P("collection.song_one"):P("collection.song_many",{count:e.tracks.length}),()=>P("collection.play_all"),()=>P("artist.shuffle"),()=>P("collection.col_title"),()=>P("collection.col_album")]),M("click",l,function(...D){e.onBack?.apply(this,D)}),M("click",w,a),M("click",R,i),h(t,o),ze()}Ve(["click","keydown"]);var zd=L('<div class="flex items-center gap-1.5"><button type="button" class="w-8 h-8 rounded-full inline-flex items-center justify-center bg-[var(--surface)] border border-[var(--border)] hover:bg-[var(--surface-hover)] hover:border-[var(--border-strong)] disabled:opacity-30 transition-all active:scale-95" title="Scroll left" aria-label="Scroll left"><!></button> <button type="button" class="w-8 h-8 rounded-full inline-flex items-center justify-center bg-[var(--surface)] border border-[var(--border)] hover:bg-[var(--surface-hover)] hover:border-[var(--border-strong)] disabled:opacity-30 transition-all active:scale-95" title="Scroll right" aria-label="Scroll right"><!></button></div>'),Nd=L('<div class="flex items-center justify-between gap-2"><h2 class="section-title"> </h2> <!></div>'),Md=L('<button type="button" class="hidden md:inline-flex items-center justify-center w-9 h-9 rounded-full absolute left-2 top-1/2 -translate-y-1/2 bg-[var(--bg-elevated)]/90 backdrop-blur-md border border-[var(--border-strong)] shadow-xl opacity-0 group-hover:opacity-100 transition-all duration-200 hover:bg-[var(--accent)] hover:text-white hover:border-transparent active:scale-95" aria-label="Scroll left"><!></button>'),Ed=L('<button type="button" class="hidden md:inline-flex items-center justify-center w-9 h-9 rounded-full absolute right-2 top-1/2 -translate-y-1/2 bg-[var(--bg-elevated)]/90 backdrop-blur-md border border-[var(--border-strong)] shadow-xl opacity-0 group-hover:opacity-100 transition-all duration-200 hover:bg-[var(--accent)] hover:text-white hover:border-transparent active:scale-95" aria-label="Scroll right"><!></button>'),Cd=L('<section class="flex flex-col gap-3"><!> <div class="relative group"><div class="carousel-track scroll-smooth svelte-e7zbyu"><!></div> <!> <!></div></section>');function Er(t,e){Ae(e,!0);let r=et(e,"minCardWidth",3,172),a=et(e,"gap",3,12),i=G(null),o=G(!1),s=G(!1),l=G(5),v=G(176);function f(){if(!n(i))return;const R=n(i).clientWidth;if(R>0){const p=Math.max(2,Math.floor((R+a())/(r()+a())));z(l,p,!0),z(v,(R-a()*(p-1))/p)}x()}function x(){if(!n(i))return;const{scrollLeft:R,scrollWidth:p,clientWidth:j}=n(i);z(o,R>4),z(s,R+j<p-4)}function g(R){if(!n(i))return;const p=n(l)*(n(v)+a());n(i).scrollBy({left:R*p,behavior:"smooth"})}Sa(()=>{if(f(),!n(i))return;const R=new ResizeObserver(()=>f());R.observe(n(i));const p=new MutationObserver(()=>x());return p.observe(n(i),{childList:!0}),()=>{R.disconnect(),p.disconnect()}});var k=Cd(),_=u(k);{var S=R=>{var p=Nd(),j=u(p),E=T(j,!0),C=d(j,2);{var q=U=>{var X=zd(),B=u(X),W=u(B);_i(W,{size:16});var H=d(B,2),F=u(H);An(F,{size:16}),O(()=>{B.disabled=!n(o),H.disabled=!n(s)}),M("click",B,()=>g(-1)),M("click",H,()=>g(1)),h(U,X)};Q(C,U=>{(n(o)||n(s))&&U(q)})}O(()=>m(E,e.title)),h(R,p)};Q(_,R=>{e.title&&R(S)})}var I=d(_,2),$=u(I),y=u($);Zl(y,()=>e.children),Xr($,R=>z(i,R),()=>n(i));var b=d($,2);{var w=R=>{var p=Md(),j=u(p);_i(j,{size:18}),M("click",p,()=>g(-1)),h(R,p)};Q(b,R=>{n(o)&&R(w)})}var A=d(b,2);{var N=R=>{var p=Ed(),j=u(p);An(j,{size:18}),M("click",p,()=>g(1)),h(R,p)};Q(A,R=>{n(s)&&R(N)})}O(()=>{le(k,"id",e.id),Ke($,`--carousel-card-w: ${n(v)??""}px; --carousel-gap: ${a()??""}px;`)}),_t("scroll",$,x),h(t,k),ze()}Ve(["click"]);var Pi=L('<button class="p-2.5 rounded-2xl bg-[var(--surface)] border border-[var(--border)] hover:bg-[var(--surface-hover)] hover:border-[var(--border-strong)] text-left flex flex-col gap-1.5 transition-colors"><!> <strong class="text-[13.5px] truncate w-full"> </strong> <span class="text-[12px] text-[var(--text-muted)] truncate w-full"> </span></button>'),Id=L('<span class="text-[11.5px] text-[var(--text-muted)]"> </span>'),Ld=L('<button class="p-2.5 rounded-2xl flex flex-col items-center gap-1.5 text-center hover:bg-[var(--surface)] transition-colors"><!> <strong class="text-[13px] truncate w-full"> </strong> <!></button>'),Rd=L('<section class="max-w-3xl rounded-2xl p-5 bg-[var(--surface)] border border-[var(--border)] space-y-2"><h3 class="text-[15px] font-bold"> </h3> <p class="text-[13.5px] leading-relaxed text-[var(--text-muted)]"> </p></section>'),jd=L("<!> <!> <!> <!>",1);function qd(t,e){Ae(e,!0);var r=jd(),a=K(r);{var i=g=>{{let k=J(()=>P("artist.albums"));Er(g,{get title(){return n(k)},children:(_,S)=>{var I=se(),$=K(I);Re($,17,()=>e.details.albums,y=>y.id,(y,b)=>{var w=Pi(),A=u(w);nt(A,{class:"media-cover",get src(){return n(b).cover_url},get alt(){return n(b).title}});var N=d(A,2),R=T(N,!0),p=d(N,2),j=T(p,!0);O(()=>{m(R,n(b).title),m(j,n(b).year??n(b).artist)}),M("click",w,()=>c.openAlbum(n(b).id,n(b).title,n(b).cover_url)),h(y,w)}),h(_,I)},$$slots:{default:!0}})}};Q(a,g=>{e.details.albums.length>0&&g(i)})}var o=d(a,2);{var s=g=>{{let k=J(()=>P("artist.playlists"));Er(g,{get title(){return n(k)},children:(_,S)=>{var I=se(),$=K(I);Re($,17,()=>e.details.playlists,y=>y.id,(y,b)=>{var w=Pi(),A=u(w);nt(A,{class:"media-cover",get src(){return n(b).cover_url},get alt(){return n(b).name}});var N=d(A,2),R=T(N,!0),p=d(N,2),j=T(p,!0);O(()=>{m(R,n(b).name),m(j,n(b).subtitle)}),M("click",w,()=>c.openAlbum(n(b).id,n(b).name,n(b).cover_url,!0)),h(y,w)}),h(_,I)},$$slots:{default:!0}})}};Q(o,g=>{e.details.playlists.length>0&&g(s)})}var l=d(o,2);{var v=g=>{{let k=J(()=>P("artist.similar"));Er(g,{get title(){return n(k)},minCardWidth:144,children:(_,S)=>{var I=se(),$=K(I);Re($,17,()=>e.details.similar_artists,y=>y.id,(y,b)=>{var w=Ld(),A=u(w);nt(A,{class:"artist-avatar",get src(){return n(b).avatar_url},get alt(){return n(b).name},fallbackIcon:"👤"});var N=d(A,2),R=T(N,!0),p=d(N,2);{var j=E=>{var C=Id(),q=T(C,!0);O(U=>m(q,U),[()=>Da(n(b).subscriber_count)]),h(E,C)};Q(p,E=>{n(b).subscriber_count&&E(j)})}O(()=>m(R,n(b).name)),M("click",w,()=>c.openArtist(n(b).id,n(b).name)),h(y,w)}),h(_,I)},$$slots:{default:!0}})}};Q(l,g=>{e.details.similar_artists.length>0&&g(v)})}var f=d(l,2);{var x=g=>{var k=Rd(),_=u(k),S=T(_,!0),I=d(_,2),$=T(I,!0);O(y=>{m(S,y),m($,e.details.description)},[()=>P("artist.about",{name:e.details.name})]),h(g,k)};Q(f,g=>{e.details.description&&g(x)})}h(t,r),ze()}Ve(["click"]);var Od=L(`<div class="absolute inset-0 pointer-events-none" style="-webkit-mask-image: linear-gradient(180deg, #000 0%, #000 54%, rgba(0,0,0,0.72) 72%, rgba(0,0,0,0.25) 88%, transparent 100%);
             mask-image: linear-gradient(180deg, #000 0%, #000 54%, rgba(0,0,0,0.72) 72%, rgba(0,0,0,0.25) 88%, transparent 100%);"><img alt="" loading="lazy" decoding="async" referrerpolicy="no-referrer"/></div>`),Dd=L('<p class="text-[13.5px] font-medium text-white/80 drop-shadow-sm"> </p>'),Fd=L('<button type="button" class="mt-1 text-[11.5px] font-bold uppercase tracking-wider text-white/90 hover:text-[var(--accent-bright)] transition-colors"> </button>'),Hd=L('<div class="pt-0.5 max-w-xl"><p> </p> <!></div>'),Vd=L('<span class="w-3.5 h-3.5 rounded-full border-2 border-current/30 border-t-current animate-spin"></span>'),Qd=L('<section class="relative w-full min-h-[340px] sm:min-h-[440px] flex flex-col justify-between overflow-hidden"><!> <div class="absolute inset-0 pointer-events-none" style="background: linear-gradient(180deg, rgba(12, 12, 15, 0.14) 0%, rgba(12, 12, 15, 0.1) 36%, rgba(12, 12, 15, 0.62) 70%, rgba(12, 12, 15, 0.25) 90%, transparent 100%);"></div> <div class="relative z-10 w-full max-w-[1280px] mx-auto px-6 pt-4 pb-8 flex-1 flex flex-col justify-between"><div><button class="w-9 h-9 rounded-full inline-flex items-center justify-center bg-black/45 hover:bg-black/65 backdrop-blur-md border border-white/15 text-white transition-all active:scale-95"><!></button></div> <div class="mt-14 sm:mt-24 max-w-2xl space-y-2.5"><h1 class="text-[38px] sm:text-[50px] font-black tracking-[-0.035em] leading-[1.04] text-white drop-shadow-[0_2px_14px_rgba(0,0,0,0.45)]"> </h1> <!> <!> <div class="flex flex-wrap items-center gap-2.5 pt-2"><button class="pill-btn primary !px-5 !py-2 !text-[13.5px] shadow-[0_6px_20px_rgba(33,144,164,0.35)]"><!> </button> <button class="pill-btn !px-4 !py-2 !text-[13.5px] bg-white/[0.08] hover:bg-white/[0.14] border-white/15 backdrop-blur-md"><!> </button> <button class="pill-btn !px-4 !py-2 !text-[13.5px] bg-white/[0.08] hover:bg-white/[0.14] border-white/15 backdrop-blur-md"><!> </button></div></div></div></section>');function Ud(t,e){Ae(e,!0);let r=G(!1),a=G(!1),i=G(!1);$t(()=>{e.details?.header_url,z(r,!1),z(i,!1)});function o(){if(!e.details?.tracks.length)return;const F=[...e.details.tracks].sort(()=>Math.random()-.5);c.replaceQueueAndPlay(F,0,e.details.name)}async function s(){if(!(!e.details?.tracks.length||n(a)||!Ue())){z(a,!0);try{const F=e.details.tracks[0],D=await Ne.getRadioTracks(F.id),V=D.length>0?[F,...D.filter(Z=>Z.id!==F.id)]:e.details.tracks;c.replaceQueueAndPlay(V,0,`Mix · ${e.details.name}`)}catch(F){c.showToast(String(F))}finally{z(a,!1)}}}var l=Qd(),v=u(l);{var f=F=>{var D=Od(),V=T(D);O(()=>{ee(V,1,`w-full h-full object-cover object-[50%_22%] transition-opacity duration-500 ${n(r)?"opacity-100":"opacity-0"}`),le(V,"src",e.details.header_url)}),_t("load",V,()=>z(r,!0)),h(F,D)};Q(v,F=>{e.details?.header_url&&F(f)})}var x=d(v,4),g=u(x),k=u(g),_=u(k);Ga(_,{size:17});var S=d(g,2),I=u(S),$=T(I,!0),y=d(I,2);{var b=F=>{var D=Dd(),V=T(D,!0);O(Z=>m(V,Z),[()=>Da(e.details.subscriber_count)]),h(F,D)};Q(y,F=>{e.details?.subscriber_count&&F(b)})}var w=d(y,2);{var A=F=>{var D=Hd(),V=u(D),Z=T(V,!0),Y=d(V,2);{var pe=de=>{var oe=Fd(),re=T(oe,!0);O(ae=>m(re,ae),[()=>n(i)?P("artist.show_less"):P("artist.more")]),M("click",oe,()=>z(i,!n(i))),h(de,oe)};Q(Y,de=>{e.details.description.length>140&&de(pe)})}O(()=>{ee(V,1,`text-[13.5px] leading-relaxed text-white/85 drop-shadow-sm ${n(i)?"":"line-clamp-2"}`),m(Z,e.details.description)}),h(F,D)};Q(w,F=>{e.details?.description&&F(A)})}var N=d(w,2),R=u(N),p=u(R);bt(p,{size:15,fill:"currentColor"});var j=d(p),E=d(R,2),C=u(E);ta(C,{size:15});var q=d(C),U=d(E,2),X=u(U);{var B=F=>{var D=Vd();h(F,D)},W=F=>{$a(F,{size:15})};Q(X,F=>{n(a)?F(B):F(W,-1)})}var H=d(X);O((F,D,V,Z)=>{le(k,"title",F),m($,e.details?.name??e.artistName),R.disabled=!e.details?.tracks.length,m(j,` ${D??""}`),E.disabled=!e.details?.tracks.length,m(q,` ${V??""}`),U.disabled=!e.details?.tracks.length||n(a),m(H,` ${Z??""}`)},[()=>P("nav.back"),()=>P("artist.play"),()=>P("artist.shuffle"),()=>P("artist.mix")]),M("click",k,()=>c.navigateTab("explore")),M("click",R,()=>e.details&&c.replaceQueueAndPlay(e.details.tracks,0,e.details.name)),M("click",E,o),M("click",U,()=>void s()),h(t,l),ze()}Ve(["click"]);var Bd=L('<div class="absolute inset-0 rounded-[6px] bg-black/55 flex items-center justify-center text-[var(--accent-bright)]"><!></div>'),Wd=L('<div class="absolute inset-0 rounded-[6px] bg-black/55 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center text-white"><!></div>'),Gd=L('<div role="button" tabindex="0"><div class="flex items-center gap-3.5 min-w-0"><div class="relative shrink-0"><!> <!></div> <div class="min-w-0"><div> </div> <div class="text-[12px] text-[var(--text-muted)] truncate md:hidden"> </div></div></div> <span class="hidden md:block text-[13.5px] text-[var(--text-muted)] truncate"> </span> <span class="hidden lg:block text-[13.5px] text-[var(--text-muted)] truncate"> </span> <div class="flex items-center gap-1 shrink-0"><button type="button" class="w-8 h-8 rounded-full inline-flex items-center justify-center text-[var(--text-muted)] opacity-0 group-hover:opacity-100 focus-visible:opacity-100 hover:text-[var(--text)] hover:bg-white/[0.1] transition-all"><!></button> <button type="button"><!></button> <span class="text-[13px] text-[var(--text-muted)] numeric w-11 text-right"> </span></div></div>'),Yd=L('<button type="button" class="inline-flex items-center justify-center px-4 py-1.5 rounded-full text-[13px] font-semibold border border-white/15 hover:bg-white/[0.08] transition-colors"> </button>'),Kd=L('<section class="space-y-4"><h2 class="text-[22px] font-extrabold tracking-[-0.02em]"> </h2> <div class="flex flex-col"></div> <!></section>');function Jd(t,e){Ae(e,!0);let r=J(()=>e.tracks.slice(0,5));var a=se(),i=K(a);{var o=s=>{var l=Kd(),v=u(l),f=T(v,!0),x=d(v,2);Re(x,23,()=>n(r),_=>_.id,(_,S,I)=>{const $=J(()=>c.currentTrack?.id===n(S).id),y=J(()=>c.isFavorite(n(S).id));var b=Gd(),w=u(b),A=u(w),N=u(A);nt(N,{class:"track-cover !w-9 !h-9 !rounded-[6px]",get src(){return n(S).cover_url},alt:""});var R=d(N,2);{var p=re=>{var ae=Bd(),ge=u(ae);vr(ge,{size:15}),h(re,ae)},j=re=>{var ae=Wd(),ge=u(ae);bt(ge,{size:14,fill:"currentColor",class:"translate-x-[1px]"}),h(re,ae)};Q(R,re=>{n($)?re(p):re(j,-1)})}var E=d(A,2),C=u(E),q=T(C,!0),U=d(C,2),X=T(U),B=d(w,2),W=T(B,!0),H=d(B,2),F=T(H,!0),D=d(H,2),V=u(D),Z=u(V);Zr(Z,{size:16});var Y=d(V,2),pe=u(Y);{let re=J(()=>n(y)?"currentColor":"none");gt(pe,{size:16,get fill(){return n(re)}})}var de=d(Y,2),oe=T(de,!0);O((re,ae,ge)=>{ee(b,1,`group grid grid-cols-[minmax(0,1fr)_auto] md:grid-cols-[minmax(0,1.6fr)_minmax(0,0.85fr)_auto] lg:grid-cols-[minmax(0,1.65fr)_minmax(0,0.85fr)_minmax(0,1fr)_auto] items-center gap-6 py-2.5 px-2.5 border-b border-white/[0.08] last:border-b-0 cursor-pointer transition-colors ${n($)?"bg-[var(--accent-soft)]":"hover:bg-white/[0.05]"}`),ee(C,1,`text-[14px] font-semibold truncate ${n($)?"text-[var(--accent-bright)] font-bold":""}`),m(q,n(S).title),m(X,`${n(S).artist??""}${n(S).album?` • ${n(S).album}`:""}`),m(W,n(S).artist),m(F,n(S).album??""),le(V,"title",re),ee(Y,1,`w-8 h-8 rounded-full inline-flex items-center justify-center transition-all ${n(y)?"text-[var(--fav-coral)] opacity-100 hover:bg-white/[0.08]":"text-[var(--text-muted)] opacity-0 group-hover:opacity-100 focus-visible:opacity-100 hover:text-[var(--text)] hover:bg-white/[0.1]"}`),le(Y,"title",ae),m(oe,ge)},[()=>P("queue.add"),()=>n(y)?P("player.favorite_remove"):P("player.favorite_add"),()=>zt(n(S).duration_secs)]),M("click",b,()=>c.replaceQueueAndPlay(e.tracks,n(I),e.artistName)),M("keydown",b,re=>re.key==="Enter"&&c.replaceQueueAndPlay(e.tracks,n(I),e.artistName)),M("click",V,re=>{re.stopPropagation(),c.enqueueTrack(n(S))}),M("click",Y,re=>{re.stopPropagation(),c.toggleFavorite(n(S))}),h(_,b)});var g=d(x,2);{var k=_=>{var S=Yd(),I=T(S,!0);O($=>m(I,$),[()=>P("artist.show_all",{count:e.tracks.length})]),M("click",S,function(...$){e.onShowAll?.apply(this,$)}),h(_,S)};Q(g,_=>{e.tracks.length>5&&e.onShowAll&&_(k)})}O(_=>m(f,_),[()=>P("artist.popular")]),h(s,l)};Q(i,s=>{n(r).length>0&&s(o)})}h(t,a),ze()}Ve(["click","keydown"]);var Xd=L('<div class="flex items-center gap-4 py-3"><div class="w-9 h-9 rounded-[6px] bg-white/[0.08]"></div> <div class="flex-1 space-y-2"><div class="h-3.5 w-48 rounded bg-white/[0.09]"></div> <div class="h-3 w-32 rounded bg-white/[0.05]"></div></div> <div class="h-3 w-28 rounded bg-white/[0.05] hidden lg:block"></div> <div class="h-3 w-10 rounded bg-white/[0.06]"></div></div>'),Zd=L('<div class="space-y-6 animate-pulse pt-2" aria-busy="true"><div class="h-5 w-40 rounded bg-white/[0.08]"></div> <div class="divide-y divide-white/[0.06]"></div></div>'),eu=L("<!> <!>",1),tu=L('<div class="w-full pb-28"><!> <div class="w-full max-w-[1280px] mx-auto px-6 space-y-8"><!></div></div>');function ru(t,e){Ae(e,!0);let r=G(null),a=G(!1),i=G(!1);$t(()=>{const g=e.artistId,k=e.artistName;z(r,null),z(i,!1),Ue()&&(z(a,!0),Ne.getArtist(g,k).then(_=>{e.artistId===g&&z(r,_,!0)}).catch(_=>c.showToast(String(_))).finally(()=>{e.artistId===g&&z(a,!1)}))});function o(){z(i,!0),document.querySelector("main")?.scrollTo({top:0,behavior:"smooth"})}function s(){z(i,!1),document.querySelector("main")?.scrollTo({top:0,behavior:"smooth"})}var l=se(),v=K(l);{var f=g=>{Ad(g,{get artistName(){return n(r).name},get tracks(){return n(r).tracks},onBack:s})},x=g=>{var k=tu(),_=u(k);Ud(_,{get artistName(){return e.artistName},get details(){return n(r)}});var S=d(_,2),I=u(S);{var $=b=>{var w=Zd(),A=d(u(w),2);Re(A,20,()=>[0,1,2,3,4],Rt,(N,R)=>{var p=Xd();h(N,p)}),h(b,w)},y=b=>{var w=eu(),A=K(w);Jd(A,{get artistName(){return n(r).name},get tracks(){return n(r).tracks},onShowAll:o});var N=d(A,2);qd(N,{get details(){return n(r)}}),h(b,w)};Q(I,b=>{n(a)?b($):n(r)&&b(y,1)})}h(g,k)};Q(v,g=>{n(i)&&n(r)?g(f):g(x,-1)})}h(t,l),ze()}const au=["music","heart","star","flame","zap","moon","sun","headphones","disc","radio","sparkles","coffee","car","dumbbell","rain","compass"];function Wr(t,e){Ae(e,!0);let r=et(e,"name",3,"music"),a=et(e,"size",3,18),i=J(()=>{const p=(r()??"music").trim().toLowerCase();return p==="♥"||p==="heart"?"heart":p==="★"||p==="star"?"star":p==="🔥"||p==="flame"?"flame":p==="⚡"||p==="zap"?"zap":p==="🌙"||p==="moon"?"moon":p==="☀"||p==="sun"?"sun":p==="🎧"||p==="headphones"?"headphones":p==="disc"?"disc":p==="radio"?"radio":p==="sparkles"?"sparkles":p==="coffee"?"coffee":p==="🚗"||p==="car"?"car":p==="🏋"||p==="dumbbell"?"dumbbell":p==="rain"?"rain":p==="compass"?"compass":"music"});var o=se(),s=K(o);{var l=p=>{gt(p,{get size(){return a()},fill:"currentColor"})},v=p=>{Uc(p,{get size(){return a()},fill:"currentColor"})},f=p=>{Cc(p,{get size(){return a()}})},x=p=>{Zs(p,{get size(){return a()}})},g=p=>{Un(p,{get size(){return a()}})},k=p=>{Xs(p,{get size(){return a()}})},_=p=>{Ws(p,{get size(){return a()}})},S=p=>{Us(p,{get size(){return a()}})},I=p=>{$a(p,{get size(){return a()}})},$=p=>{Qc(p,{get size(){return a()}})},y=p=>{Nc(p,{get size(){return a()}})},b=p=>{Vs(p,{get size(){return a()}})},w=p=>{Ec(p,{get size(){return a()}})},A=p=>{zc(p,{get size(){return a()}})},N=p=>{xa(p,{get size(){return a()}})},R=p=>{ea(p,{get size(){return a()}})};Q(s,p=>{n(i)==="heart"?p(l):n(i)==="star"?p(v,1):n(i)==="flame"?p(f,2):n(i)==="zap"?p(x,3):n(i)==="moon"?p(g,4):n(i)==="sun"?p(k,5):n(i)==="headphones"?p(_,6):n(i)==="disc"?p(S,7):n(i)==="radio"?p(I,8):n(i)==="sparkles"?p($,9):n(i)==="coffee"?p(y,10):n(i)==="car"?p(b,11):n(i)==="dumbbell"?p(w,12):n(i)==="rain"?p(A,13):n(i)==="compass"?p(N,14):p(R,-1)})}h(t,o),ze()}var nu=L('<p class="text-[13px] text-[var(--text-muted)] leading-snug"> </p>'),iu=L('<button class="w-11 h-11 rounded-full inline-flex items-center justify-center bg-white/[0.08] hover:bg-white/[0.15] border border-white/10 transition-all active:scale-95"><!></button>'),su=L('<button class="w-11 h-11 rounded-full inline-flex items-center justify-center bg-white/[0.08] hover:bg-white/[0.15] border border-white/10 transition-all active:scale-95 disabled:opacity-35 disabled:cursor-not-allowed"><!></button>'),ou=L('<button class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-[12px] font-semibold text-[var(--text-muted)] hover:text-red-400 hover:bg-red-500/15 transition-colors mt-1"><!> </button>'),lu=L('<aside class="lg:sticky lg:top-8 flex flex-col items-center text-center gap-4"><div class="w-full flex items-center justify-start"><button class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-[12.5px] font-semibold text-[var(--text-muted)] hover:text-[var(--text)] bg-white/[0.06] hover:bg-white/[0.12] border border-white/10 transition-all active:scale-95"><!> </button></div> <div class="relative w-[248px] h-[248px] rounded-2xl flex items-center justify-center text-white overflow-hidden border border-white/15 shadow-[0_24px_56px_rgba(0,0,0,0.5)]"><div class="pointer-events-none absolute inset-0 bg-gradient-to-br from-white/20 via-transparent to-black/30"></div> <div class="relative z-10 drop-shadow-[0_10px_24px_rgba(0,0,0,0.35)]"><!></div></div> <div class="space-y-1.5 max-w-[300px]"><h1 class="text-[26px] font-black tracking-[-0.025em] leading-tight"> </h1> <!> <div class="flex items-center justify-center gap-2 text-[12.5px] font-medium text-[var(--text-muted)] pt-0.5"><span class="px-2.5 py-0.5 rounded-full text-[10.5px] font-bold uppercase tracking-wider bg-white/[0.08] text-[var(--text)]/90 border border-white/10"> </span> <span>·</span> <span class="numeric"> </span></div></div> <div class="flex items-center justify-center gap-4 pt-1"><button class="w-11 h-11 rounded-full inline-flex items-center justify-center bg-white/[0.08] hover:bg-white/[0.15] border border-white/10 transition-all active:scale-95 disabled:opacity-35 disabled:cursor-not-allowed"><!></button> <button class="play-circle !w-[60px] !h-[60px] disabled:opacity-35 disabled:cursor-not-allowed"><!></button> <!></div> <!></aside>');function cu(t,e){Ae(e,!0);let r=et(e,"playlist",3,null);function a(){if(e.tracks.length){for(const D of e.tracks)c.queue=[...c.queue,D];c.persistSession(),c.showToast(`${e.tracks.length} ${P("queue.title")}`)}}var i=lu(),o=u(i),s=u(o),l=u(s);Ga(l,{size:15});var v=d(l),f=d(o,2),x=d(u(f),2),g=u(x);{var k=D=>{gt(D,{size:82,fill:"currentColor"})},_=D=>{{let V=J(()=>r()?.icon_name);Wr(D,{get name(){return n(V)},size:82})}};Q(g,D=>{e.mode==="favorites"?D(k):D(_,-1)})}var S=d(f,2),I=u(S),$=T(I,!0),y=d(I,2);{var b=D=>{var V=nu(),Z=T(V,!0);O(()=>m(Z,e.description)),h(D,V)};Q(y,D=>{e.description&&D(b)})}var w=d(y,2),A=u(w),N=T(A,!0),R=d(A,4),p=T(R),j=d(S,2),E=u(j),C=u(E);ta(C,{size:18});var q=d(E,2),U=u(q);bt(U,{size:24,fill:"currentColor",class:"translate-x-[1.5px]"});var X=d(q,2);{var B=D=>{var V=iu(),Z=u(V);Ks(Z,{size:17}),O(Y=>le(V,"title",Y),[()=>P("collection.customize")]),M("click",V,()=>e.onEdit?.(r())),h(D,V)},W=D=>{var V=su(),Z=u(V);Zr(Z,{size:18}),O(Y=>{V.disabled=e.tracks.length===0,le(V,"title",Y)},[()=>P("queue.add")]),M("click",V,a),h(D,V)};Q(X,D=>{e.mode==="playlist"&&r()?D(B):D(W,-1)})}var H=d(j,2);{var F=D=>{var V=ou(),Z=u(V);Kn(Z,{size:13});var Y=d(Z);O(pe=>m(Y,` ${pe??""}`),[()=>P("collection.delete")]),M("click",V,()=>e.onDelete?.(r())),h(D,V)};Q(H,D=>{e.mode==="playlist"&&r()&&D(F)})}O((D,V,Z,Y,pe,de)=>{le(s,"title",D),m(v,` ${V??""}`),Ke(f,e.mode==="favorites"?"background: linear-gradient(135deg, #e01b24 0%, #ff6b81 100%);":`background: linear-gradient(135deg, ${e.accentColor} 0%, ${e.accentColor}b8 100%);`),m($,e.title),m(N,Z),m(p,`${Y??""}${e.totalMins?` · ${e.totalMins} min`:""}`),E.disabled=e.tracks.length===0,le(E,"title",pe),q.disabled=e.tracks.length===0,le(q,"title",de)},[()=>P("nav.back"),()=>P("nav.back"),()=>e.mode==="favorites"?P("collection.favorites"):P("collection.local_badge"),()=>e.tracks.length===1?P("collection.song_one"):P("collection.song_many",{count:e.tracks.length}),()=>P("artist.shuffle"),()=>P("collection.play_all")]),M("click",s,function(...D){e.onBack?.apply(this,D)}),M("click",E,function(...D){e.onShuffle?.apply(this,D)}),M("click",q,function(...D){e.onPlayAll?.apply(this,D)}),h(t,i),ze()}Ve(["click"]);var du=L('<div class="min-h-[340px] flex flex-col items-center justify-center text-center px-6 py-12 rounded-2xl bg-white/[0.025] border border-dashed border-white/[0.1]"><div class="w-14 h-14 rounded-2xl flex items-center justify-center text-white mb-4 shadow-lg border border-white/15"><!></div> <h2 class="text-[17px] font-bold tracking-[-0.01em] mb-1.5"> </h2> <p class="text-[13.5px] text-[var(--text-muted)] max-w-[340px] leading-relaxed mb-5"> </p> <button class="pill-btn primary !px-4 !py-2"><!> </button></div>'),uu=L('<div class="flex items-center justify-end"><button class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-[12px] font-semibold bg-white/[0.05] hover:bg-white/[0.1] border border-white/10 transition-colors"><!> </button></div>'),vu=L('<span class="text-[12.5px] font-semibold text-[var(--text-muted)] numeric group-hover:hidden"> </span> <!>',1),fu=L('<button class="w-8 h-8 rounded-full inline-flex items-center justify-center text-[var(--text-muted)] hover:text-red-400 hover:bg-red-500/15 transition-colors"><!></button>'),hu=L('<div role="button" tabindex="0"><div class="w-6 flex items-center justify-center shrink-0"><!></div> <!> <div class="min-w-0 flex-1"><div> </div> <div class="text-[12px] text-[var(--text-muted)] truncate"> </div></div> <div class="flex items-center gap-1 shrink-0"><div class="flex items-center gap-0.5 opacity-0 group-hover:opacity-100 focus-within:opacity-100 transition-opacity duration-150"><button class="w-8 h-8 rounded-full inline-flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text)] hover:bg-white/[0.1] transition-colors"><!></button> <!></div> <button><!></button> <span class="text-[12.5px] text-[var(--text-muted)] numeric w-11 text-right"> </span></div></div>'),pu=L('<!> <div><div class="flex items-center gap-4 px-3 pb-2.5 text-[11px] font-bold uppercase tracking-wider text-[var(--text-muted)]/75 border-b border-white/[0.08]"><span class="w-6 text-center">#</span> <span class="flex-1"> </span> <span class="w-11 flex justify-end"><!></span></div> <div class="flex flex-col"></div></div>',1),_u=L('<section class="min-w-0 flex flex-col gap-3"><!></section>');function gu(t,e){Ae(e,!0);let r=et(e,"playlist",3,null);function a(v){return P(v==="title"?"collection.sort_title":v==="artist"?"collection.sort_artist":v==="duration"?"collection.sort_duration":"collection.sort_custom")}var i=_u(),o=u(i);{var s=v=>{var f=du(),x=u(f),g=u(x);{var k=N=>{gt(N,{size:24,fill:"currentColor"})},_=N=>{ea(N,{size:24})};Q(g,N=>{e.mode==="favorites"?N(k):N(_,-1)})}var S=d(x,2),I=T(S,!0),$=d(S,2),y=T($,!0),b=d($,2),w=u(b);xa(w,{size:15});var A=d(w);O((N,R,p)=>{Ke(x,`background: linear-gradient(135deg, ${e.accentColor??""}cc, ${e.accentColor??""}66);`),m(I,N),m(y,R),m(A,` ${p??""}`)},[()=>e.mode==="favorites"?P("collection.empty_favorites_title"):P("collection.empty_title"),()=>e.mode==="favorites"?P("collection.empty_favorites"):P("collection.empty_playlist"),()=>P("collection.explore_music")]),M("click",b,()=>c.navigateTab("explore")),h(v,f)},l=v=>{var f=pu(),x=K(f);{var g=w=>{var A=uu(),N=u(A),R=u(N);$c(R,{size:13});var p=d(R);O(j=>m(p,` ${j??""}`),[()=>P("collection.sort",{mode:a(e.sortBy)})]),M("click",N,function(...j){e.onCycleSort?.apply(this,j)}),h(w,A)};Q(x,w=>{e.tracks.length>1&&w(g)})}var k=d(x,2),_=u(k),S=d(u(_),2),I=T(S,!0),$=d(S,2),y=u($);Ya(y,{size:13});var b=d(_,2);Re(b,23,()=>e.tracks,(w,A)=>w.id+":"+A,(w,A,N)=>{const R=J(()=>c.currentTrack?.id===n(A).id),p=J(()=>c.isFavorite(n(A).id));var j=hu(),E=u(j),C=u(E);{var q=ce=>{vr(ce,{size:15,class:"text-[var(--accent-bright)]"})},U=ce=>{var be=vu(),me=K(be),Ee=T(me,!0),we=d(me,2);bt(we,{size:13,fill:"currentColor",class:"hidden group-hover:block translate-x-[1px]"}),O(()=>m(Ee,n(N)+1)),h(ce,be)};Q(C,ce=>{n(R)?ce(q):ce(U,-1)})}var X=d(E,2);nt(X,{class:"track-cover !w-10 !h-10 !rounded-[8px] shrink-0",get src(){return n(A).cover_url},get alt(){return e.title}});var B=d(X,2),W=u(B),H=T(W,!0),F=d(W,2),D=T(F),V=d(B,2),Z=u(V),Y=u(Z),pe=u(Y);Zr(pe,{size:16});var de=d(Y,2);{var oe=ce=>{var be=fu(),me=u(be);Kn(me,{size:15}),O(Ee=>le(be,"title",Ee),[()=>P("collection.remove_from_playlist")]),M("click",be,Ee=>{Ee.stopPropagation(),c.removeTrackFromPlaylist(r().id,n(A).id)}),h(ce,be)};Q(de,ce=>{e.mode==="playlist"&&r()&&ce(oe)})}var re=d(Z,2),ae=u(re);{let ce=J(()=>n(p)?"currentColor":"none");gt(ae,{size:16,get fill(){return n(ce)}})}var ge=d(re,2),Te=T(ge,!0);O((ce,be,me)=>{ee(j,1,`group flex items-center gap-3.5 py-2.5 px-3 border-b border-white/[0.06] last:border-b-0 cursor-pointer transition-colors ${n(R)?"bg-[var(--accent-soft)]":"hover:bg-white/[0.05]"}`),ee(W,1,`text-[14px] font-semibold truncate ${n(R)?"text-[var(--accent-bright)] font-bold":""}`),m(H,n(A).title),m(D,`${n(A).artist??""}${n(A).album?` • ${n(A).album}`:""}`),le(Y,"title",ce),ee(re,1,`w-8 h-8 rounded-full inline-flex items-center justify-center transition-all ${n(p)?"text-[var(--fav-coral)] opacity-100 hover:bg-white/[0.08]":"text-[var(--text-muted)] opacity-0 group-hover:opacity-100 focus-visible:opacity-100 hover:text-[var(--text)] hover:bg-white/[0.1]"}`),le(re,"title",be),m(Te,me)},[()=>P("queue.add"),()=>n(p)?P("player.favorite_remove"):P("player.favorite_add"),()=>zt(n(A).duration_secs)]),M("click",j,()=>e.onPlayTrack(n(N))),M("keydown",j,ce=>ce.key==="Enter"&&e.onPlayTrack(n(N))),M("click",Y,ce=>{ce.stopPropagation(),c.enqueueTrack(n(A))}),M("click",re,ce=>{ce.stopPropagation(),c.toggleFavorite(n(A))}),h(w,j)}),O(w=>m(I,w),[()=>P("collection.col_title")]),h(v,f)};Q(o,v=>{e.tracks.length===0?v(s):v(l,-1)})}h(t,i),ze()}Ve(["click","keydown"]);var bu=L('<div class="w-full min-h-full transition-all duration-500"><div class="w-full max-w-[1280px] mx-auto px-4 sm:px-8 pt-4 sm:pt-6 pb-36 md:pb-28"><div class="grid grid-cols-1 lg:grid-cols-[320px_minmax(0,1fr)] gap-8 lg:gap-12 items-start"><!> <!></div></div></div>');function Ti(t,e){Ae(e,!0);let r=et(e,"playlist",3,null),a=G("custom"),i=J(()=>e.mode==="favorites"?c.favorites:r()?.tracks??[]),o=J(()=>e.mode==="favorites"?P("collection.favorites"):r()?.name??"Playlist"),s=J(()=>e.mode==="favorites"?P("collection.favorites_desc"):r()?.description??""),l=J(()=>e.mode==="favorites"?"#e01b24":r()?.color_hex||"#2190a4"),v=J(()=>{const w=[...n(i)];return n(a)==="title"?w.sort((A,N)=>A.title.localeCompare(N.title)):n(a)==="artist"?w.sort((A,N)=>A.artist.localeCompare(N.artist)):n(a)==="duration"&&w.sort((A,N)=>(A.duration_secs??0)-(N.duration_secs??0)),w}),f=J(()=>Math.round(n(i).reduce((w,A)=>w+(A.duration_secs??0),0)/60));function x(){const w=["custom","title","artist","duration"];z(a,w[(w.indexOf(n(a))+1)%w.length],!0)}function g(){n(v).length&&c.replaceQueueAndPlay(n(v),0,n(o))}function k(){if(!n(v).length)return;const w=[...n(v)].sort(()=>Math.random()-.5);c.replaceQueueAndPlay(w,0,n(o))}function _(w){c.replaceQueueAndPlay(n(v),w,n(o))}var S=bu(),I=u(S),$=u(I),y=u($);cu(y,{get mode(){return e.mode},get playlist(){return r()},get title(){return n(o)},get description(){return n(s)},get accentColor(){return n(l)},get tracks(){return n(v)},get totalMins(){return n(f)},get onBack(){return e.onBack},onPlayAll:g,onShuffle:k,get onEdit(){return e.onEdit},get onDelete(){return e.onDelete}});var b=d(y,2);gu(b,{get mode(){return e.mode},get playlist(){return r()},get title(){return n(o)},get accentColor(){return n(l)},get tracks(){return n(v)},get sortBy(){return n(a)},onCycleSort:x,onPlayTrack:_}),O(()=>Ke(S,`background-image: radial-gradient(ellipse 75% 48% at 24% 26%, ${n(l)??""}30 0%, ${n(l)??""}0d 48%, transparent 78%);`)),h(t,S),ze()}var Ai=L('<button type="button"><!></button>'),xu=L('<div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-md" role="presentation"><div role="dialog" aria-modal="true" class="w-[min(500px,calc(100vw-32px))] max-h-[calc(100vh-48px)] overflow-y-auto rounded-[18px] bg-[var(--bg-elevated)] text-[var(--text)] border border-[var(--border-strong)] shadow-[0_24px_64px_rgba(0,0,0,0.62)]"><form class="p-6 flex flex-col gap-5"><div class="flex items-center justify-between"><h3 class="text-[18px] font-extrabold tracking-tight"> </h3> <button type="button" class="icon-btn" title="Close"><!></button></div> <div class="flex items-center gap-4 p-4 rounded-2xl border border-white/10 transition-all duration-200"><div class="w-16 h-16 rounded-2xl flex items-center justify-center text-white shadow-lg shrink-0 transition-colors"><!></div> <div class="min-w-0 flex-1"><span class="text-[10px] font-extrabold uppercase tracking-[0.09em] text-[var(--accent-bright)]">Local playlist</span> <div class="text-[17px] font-extrabold truncate"> </div> <div class="text-[12px] text-[var(--text-muted)] truncate"> </div></div></div> <div class="space-y-3"><label class="flex flex-col gap-1.5 text-[12.5px] font-semibold"><span class="text-[var(--text-muted)]">Name</span> <input type="text" placeholder="Late Night Drive, Chill Mix…" required="" class="w-full px-3.5 py-2.5 rounded-xl bg-[var(--surface)] border border-[var(--border-strong)] focus:border-[var(--accent-border)] outline-none text-[14px]"/></label> <label class="flex flex-col gap-1.5 text-[12.5px] font-semibold"><span class="text-[var(--text-muted)]">Description</span> <input type="text" placeholder="Optional subtitle or vibe" class="w-full px-3.5 py-2 rounded-xl bg-[var(--surface)] border border-[var(--border)] focus:border-[var(--accent-border)] outline-none text-[13.5px]"/></label></div> <div class="space-y-2"><div class="flex items-center justify-between text-[12.5px] font-semibold"><span class="text-[var(--text-muted)]">Accent color</span> <button type="button">Auto</button></div> <div class="flex items-center flex-wrap gap-2"></div></div> <div class="space-y-2"><span class="text-[12.5px] font-semibold text-[var(--text-muted)]">Icon</span> <div class="grid grid-cols-8 gap-1.5"></div></div> <div class="flex items-center justify-end gap-2 pt-1"><button type="button" class="pill-btn">Cancel</button> <button type="submit" class="pill-btn primary !px-5"> </button></div></form></div></div>');function mu(t,e){Ae(e,!0);const r=["#2190a4","#3584e4","#9141ac","#c061cb","#e01b24","#e66100","#f5c211","#2ec27e","#0ea5e9","#64748b"];let a=G(""),i=G(""),o=G(Je(r[0])),s=G("music");$t(()=>{e.open&&(z(a,e.editing?.name??"",!0),z(i,e.editing?.description??"",!0),z(o,e.editing?.color_hex??r[0],!0),z(s,e.editing?.icon_name??"music",!0))});function l(_){e.open&&_.key==="Escape"&&(_.preventDefault(),e.onClose())}function v(_){_.preventDefault();const S=n(a).trim();S&&(e.onSave({id:e.editing?.id??null,name:S,description:n(i).trim(),colorHex:n(o),iconName:n(s)}),e.onClose())}let f=J(()=>n(o)??"#2190a4");var x=se();_t("keydown",ga,l);var g=K(x);{var k=_=>{var S=xu(),I=u(S),$=u(I),y=u($),b=u(y),w=T(b,!0),A=d(b,2),N=u(A);Cr(N,{size:17});var R=d(y,2),p=u(R),j=u(p);Wr(j,{get name(){return n(s)},size:28});var E=d(p,2),C=d(u(E),2),q=T(C,!0),U=d(C,2),X=T(U,!0),B=d(R,2),W=u(B),H=d(u(W),2),F=d(W,2),D=d(u(F),2),V=d(B,2),Z=u(V),Y=d(u(Z),2),pe=d(Z,2);Re(pe,21,()=>r,Rt,(ce,be)=>{var me=Ai(),Ee=u(me);{var we=qe=>{Qs(qe,{size:14,strokeWidth:3})};Q(Ee,qe=>{n(o)===n(be)&&qe(we)})}O(()=>{ee(me,1,`w-7 h-7 rounded-full inline-flex items-center justify-center text-white transition-transform hover:scale-110 ${n(o)===n(be)?"ring-2 ring-white ring-offset-2 ring-offset-[var(--bg-elevated)] scale-110":""}`),Ke(me,`background: ${n(be)??""};`),le(me,"aria-label",n(be))}),M("click",me,()=>z(o,n(be),!0)),h(ce,me)});var de=d(V,2),oe=d(u(de),2);Re(oe,21,()=>au,Rt,(ce,be)=>{var me=Ai(),Ee=u(me);Wr(Ee,{get name(){return n(be)},size:17}),O(()=>{ee(me,1,`h-10 rounded-xl inline-flex items-center justify-center border transition-all ${n(s)===n(be)?"bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)] scale-105":"bg-[var(--surface)] border-[var(--border)] text-[var(--text-muted)] hover:bg-[var(--surface-hover)] hover:text-[var(--text)]"}`),le(me,"title",n(be))}),M("click",me,()=>z(s,n(be),!0)),h(ce,me)});var re=d(de,2),ae=u(re),ge=d(ae,2),Te=T(ge,!0);O((ce,be)=>{le(I,"aria-label",e.editing?"Customize playlist":"New playlist"),m(w,e.editing?"Customize playlist":"New playlist"),Ke(R,`background: linear-gradient(135deg, ${n(f)??""}38, var(--surface) 75%);`),Ke(p,`background: ${n(f)??""};`),m(q,ce),m(X,be),ee(Y,1,`px-2.5 py-0.5 rounded-full text-[11px] font-bold transition-colors ${n(o)===null?"bg-[var(--accent-soft)] text-[var(--accent-bright)]":"bg-[var(--surface)] text-[var(--text-muted)] hover:text-[var(--text)]"}`),m(Te,e.editing?"Save changes":"Create playlist")},[()=>n(a).trim()||"Untitled playlist",()=>n(i).trim()||"Your curated local collection"]),M("click",S,ce=>{ce.target===ce.currentTarget&&e.onClose()}),_t("submit",$,v),M("click",A,function(...ce){e.onClose?.apply(this,ce)}),qa(H,()=>n(a),ce=>z(a,ce)),qa(D,()=>n(i),ce=>z(i,ce)),M("click",Y,()=>z(o,null)),M("click",ae,function(...ce){e.onClose?.apply(this,ce)}),ot(3,I,()=>Ir,()=>({start:.95,duration:190,easing:At})),ot(3,S,()=>wr,()=>({duration:160})),h(_,S)};Q(g,_=>{e.open&&_(k)})}h(t,x),ze()}Ve(["click"]);var yu=L('<div class="group flex items-center gap-3.5 p-3.5 rounded-2xl border border-[var(--border)] hover:border-white/20 cursor-pointer transition-all duration-150 hover:-translate-y-0.5" role="button" tabindex="0"><span class="w-12 h-12 rounded-xl inline-flex items-center justify-center text-white shrink-0 shadow-md"><!></span> <div class="min-w-0 flex-1"><strong class="block text-[14.5px] font-bold truncate"> </strong> <span class="block text-[12px] text-[var(--text-muted)] truncate"> </span></div> <div class="flex items-center gap-1 shrink-0"><button class="icon-btn !w-8 !h-8 opacity-0 group-hover:opacity-100 transition-opacity" title="Customize playlist"><!></button> <button class="w-9 h-9 rounded-full inline-flex items-center justify-center bg-white/10 text-white group-hover:bg-[var(--accent)] transition-all active:scale-95 disabled:opacity-35" title="Play playlist"><!></button></div></div>'),wu=L('<section class="mt-6 flex flex-col gap-3"><h2 class="section-title"> </h2> <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3"><div class="group flex items-center gap-3.5 p-3.5 rounded-2xl border border-white/10 hover:border-white/20 cursor-pointer transition-all duration-150 hover:-translate-y-0.5" style="background: linear-gradient(135deg, rgba(224, 27, 36, 0.2), var(--surface) 70%);" role="button" tabindex="0"><span class="w-12 h-12 rounded-xl inline-flex items-center justify-center text-white shrink-0 shadow-md" style="background: linear-gradient(135deg, #e01b24, #ff6b81);"><!></span> <div class="min-w-0 flex-1"><strong class="block text-[14.5px] font-bold truncate"> </strong> <span class="block text-[12px] text-[var(--text-muted)]"> </span></div> <button class="w-9 h-9 rounded-full inline-flex items-center justify-center shrink-0 bg-white/10 text-white group-hover:bg-[var(--accent)] transition-all active:scale-95 disabled:opacity-35" title="Play favorites"><!></button></div> <!></div></section>');function ku(t,e){Ae(e,!0);var r=wu(),a=u(r),i=T(a,!0),o=d(a,2),s=u(o),l=u(s),v=u(l);gt(v,{size:20,fill:"currentColor"});var f=d(l,2),x=u(f),g=T(x,!0),k=d(x,2),_=T(k,!0),S=d(f,2),I=u(S);bt(I,{size:15,fill:"currentColor",class:"translate-x-[1px]"});var $=d(s,2);Re($,17,()=>c.userPlaylists,y=>y.id,(y,b)=>{const w=J(()=>n(b).color_hex||"#2190a4");var A=yu(),N=u(A),R=u(N);Wr(R,{get name(){return n(b).icon_name},size:20});var p=d(N,2),j=u(p),E=T(j,!0),C=d(j,2),q=T(C),U=d(p,2),X=u(U),B=u(X);Ks(B,{size:14});var W=d(X,2),H=u(W);bt(H,{size:15,fill:"currentColor",class:"translate-x-[1px]"}),O(()=>{Ke(A,`background: linear-gradient(135deg, ${n(w)??""}26, var(--surface) 72%);`),Ke(N,`background: ${n(w)??""};`),m(E,n(b).name),m(q,`${n(b).tracks.length??""} ${n(b).tracks.length===1?"song":"songs"}${n(b).description?` · ${n(b).description}`:""}`),W.disabled=n(b).tracks.length===0}),M("click",A,()=>e.onOpenPlaylist(n(b).id)),M("keydown",A,F=>F.key==="Enter"&&e.onOpenPlaylist(n(b).id)),M("click",X,F=>{F.stopPropagation(),e.onEditPlaylist(n(b))}),M("click",W,F=>{F.stopPropagation(),n(b).tracks.length&&c.replaceQueueAndPlay(n(b).tracks,0,n(b).name)}),h(y,A)}),O((y,b,w)=>{m(i,y),m(g,b),m(_,w),S.disabled=c.favorites.length===0},[()=>P("collection.local_playlists"),()=>P("collection.favorites"),()=>c.favorites.length===1?P("collection.song_one"):P("collection.song_many",{count:c.favorites.length})]),M("click",s,function(...y){e.onOpenFavorites?.apply(this,y)}),M("keydown",s,y=>y.key==="Enter"&&e.onOpenFavorites()),M("click",S,y=>{y.stopPropagation(),c.favorites.length&&c.replaceQueueAndPlay(c.favorites,0,"Favorites")}),h(t,r),ze()}Ve(["click","keydown"]);var Su=L('<span class="text-[12.5px] font-semibold text-[var(--text-muted)] numeric"> </span>'),$u=L('<button class="pill-btn"><!> </button>'),Pu=L('<div class="py-10 text-center rounded-2xl bg-[var(--surface)] border border-[var(--border)]"><p class="text-[var(--text-muted)] text-[13.5px]"> </p></div>'),Tu=L('<div class="absolute inset-0 rounded-lg bg-black/55 flex items-center justify-center text-[var(--accent-bright)]"><!></div>'),Au=L('<div class="absolute inset-0 rounded-lg bg-black/55 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center text-white"><!></div>'),zu=L('<button class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-[12px] font-semibold bg-[var(--surface)] hover:bg-[var(--accent-soft)] hover:text-[var(--accent-bright)] border border-[var(--border)] transition-colors"><!> </button>'),Nu=L('<div class="flex items-center flex-wrap gap-2 px-4 py-2.5 bg-white/[0.03] border-t border-white/[0.05]"><span class="text-[11.5px] font-semibold text-[var(--text-muted)]">Add to playlist:</span> <!> <button class="inline-flex items-center gap-1 px-3 py-1 rounded-full text-[12px] font-semibold text-[var(--accent-bright)] bg-[var(--accent-soft)] hover:bg-[var(--accent)] hover:text-white transition-colors"><!> New playlist</button></div>'),Mu=L('<div class="border-b border-white/[0.06] last:border-b-0"><div role="button" tabindex="0"><div class="flex items-center gap-3.5 min-w-0"><div class="relative shrink-0"><!> <!></div> <div class="min-w-0"><div> </div> <div class="text-[12px] text-[var(--text-muted)] truncate"> </div></div></div> <span class="hidden md:block text-[13px] text-[var(--text-muted)] truncate"> </span> <div class="flex items-center gap-1 shrink-0"><button title="Favorite"><!></button> <button title="Add to playlist"><!></button> <button class="icon-btn !w-8 !h-8" title="Add to queue"><!></button> <span class="text-[12.5px] text-[var(--text-muted)] numeric w-11 text-right"> </span></div></div> <!></div>'),Eu=L('<div><div class="grid grid-cols-[minmax(0,1fr)_auto] md:grid-cols-[minmax(0,1.5fr)_minmax(0,1fr)_auto] items-center gap-4 px-3 pb-2.5 text-[11px] font-bold uppercase tracking-wider text-[var(--text-muted)]/75 border-b border-white/[0.08]"><span> </span> <span class="hidden md:block"> </span> <span class="w-28 flex justify-end pr-2"><!></span></div> <div class="flex flex-col"></div></div>'),Cu=L('<section class="mt-8 flex flex-col gap-3"><div class="flex items-center justify-between gap-3"><div class="flex items-baseline gap-2"><h2 class="section-title"> </h2> <!></div> <!></div> <!></section>');function Iu(t,e){Ae(e,!0);let r=G(null);function a($){z(r,n(r)===$.id?null:$.id,!0)}var i=Cu(),o=u(i),s=u(o),l=u(s),v=T(l,!0),f=d(l,2);{var x=$=>{var y=Su(),b=T(y,!0);O(()=>m(b,c.recentTracks.length)),h($,y)};Q(f,$=>{c.recentTracks.length>0&&$(x)})}var g=d(s,2);{var k=$=>{var y=$u(),b=u(y);bt(b,{size:13,fill:"currentColor"});var w=d(b);O(A=>m(w,` ${A??""}`),[()=>P("collection.play_all")]),M("click",y,()=>c.replaceQueueAndPlay(c.recentTracks,0,P("collection.recently_played"))),h($,y)};Q(g,$=>{c.recentTracks.length>0&&$(k)})}var _=d(o,2);{var S=$=>{var y=Pu(),b=u(y),w=T(b,!0);O(A=>m(w,A),[()=>P("collection.empty_recent")]),h($,y)},I=$=>{var y=Eu(),b=u(y),w=u(b),A=T(w,!0),N=d(w,2),R=T(N,!0),p=d(N,2),j=u(p);Ya(j,{size:13});var E=d(b,2);Re(E,23,()=>c.recentTracks,C=>C.id,(C,q,U)=>{const X=J(()=>c.currentTrack?.id===n(q).id);var B=Mu(),W=u(B),H=u(W),F=u(H),D=u(F);nt(D,{class:"track-cover !w-10 !h-10 !rounded-lg",get src(){return n(q).cover_url},alt:""});var V=d(D,2);{var Z=ke=>{var Me=Tu(),Oe=u(Me);vr(Oe,{size:16}),h(ke,Me)},Y=ke=>{var Me=Au(),Oe=u(Me);bt(Oe,{size:14,fill:"currentColor",class:"translate-x-[1px]"}),h(ke,Me)};Q(V,ke=>{n(X)?ke(Z):ke(Y,-1)})}var pe=d(F,2),de=u(pe),oe=T(de,!0),re=d(de,2),ae=T(re,!0),ge=d(H,2),Te=T(ge,!0),ce=d(ge,2),be=u(ce);let me;var Ee=u(be);{let ke=J(()=>c.isFavorite(n(q).id)?"currentColor":"none");gt(Ee,{size:16,get fill(){return n(ke)}})}var we=d(be,2),qe=u(we);Zr(qe,{size:16});var We=d(we,2),Ye=u(We);yr(Ye,{size:16});var vt=d(We,2),lt=T(vt,!0),ye=d(W,2);{var Pe=ke=>{var Me=Nu(),Oe=d(u(Me),2);Re(Oe,17,()=>c.userPlaylists,Ce=>Ce.id,(Ce,$e)=>{var Be=zu(),tt=u(Be);Wr(tt,{get name(){return n($e).icon_name},size:13});var Xe=d(tt);O(()=>m(Xe,` ${n($e).name??""}`)),M("click",Be,()=>{c.addTrackToPlaylist(n($e).id,n(q)),z(r,null)}),h(Ce,Be)});var ne=d(Oe,2),Se=u(ne);yr(Se,{size:13}),M("click",ne,()=>{z(r,null),e.onCreatePlaylist()}),ot(3,Me,()=>pr,()=>({y:-6,duration:150,easing:At})),h(ke,Me)};Q(ye,ke=>{n(r)===n(q).id&&ke(Pe)})}O((ke,Me)=>{ee(W,1,`group grid grid-cols-[minmax(0,1fr)_auto] md:grid-cols-[minmax(0,1.5fr)_minmax(0,1fr)_auto] items-center gap-4 py-2.5 px-3 cursor-pointer transition-colors ${n(X)?"bg-[var(--accent-soft)]":"hover:bg-white/[0.05]"}`),ee(de,1,`text-[14px] font-semibold truncate ${n(X)?"text-[var(--accent-bright)] font-bold":""}`),m(oe,n(q).title),m(ae,n(q).artist),m(Te,n(q).album??""),me=ee(be,1,"icon-btn !w-8 !h-8",null,me,{"fav-active":ke}),ee(we,1,`icon-btn !w-8 !h-8 ${n(r)===n(q).id?"active":""}`),m(lt,Me)},[()=>c.isFavorite(n(q).id),()=>zt(n(q).duration_secs)]),M("click",W,()=>c.replaceQueueAndPlay(c.recentTracks,n(U),"Recently played")),M("keydown",W,ke=>ke.key==="Enter"&&c.replaceQueueAndPlay(c.recentTracks,n(U),"Recently played")),M("click",be,ke=>{ke.stopPropagation(),c.toggleFavorite(n(q))}),M("click",we,ke=>{ke.stopPropagation(),a(n(q))}),M("click",We,ke=>{ke.stopPropagation(),c.enqueueTrack(n(q))}),h(C,B)}),O((C,q)=>{m(A,C),m(R,q)},[()=>P("collection.col_title"),()=>P("collection.col_album")]),h($,y)};Q(_,$=>{c.recentTracks.length===0?$(S):$(I,-1)})}O($=>m(v,$),[()=>P("collection.recently_played")]),h(t,i),ze()}Ve(["click","keydown"]);var Lu=L('<div class="page-clamp"><div class="flex items-center justify-between gap-3"><div><h1 class="text-[28px] font-extrabold tracking-[-0.025em] leading-tight"> </h1> <p class="text-[13px] text-[var(--text-muted)] mt-0.5"> </p></div> <button class="pill-btn primary !px-4"><!> </button></div> <!> <!></div>'),Ru=L("<!> <!>",1);function ju(t,e){Ae(e,!0);let r=G(Je({kind:"overview"})),a=G(!1),i=G(null),o=J(()=>{const $=n(r);return $.kind!=="playlist"?null:c.userPlaylists.find(y=>y.id===$.id)??null});function s(){z(i,null),z(a,!0)}function l($){z(i,$,!0),z(a,!0)}function v($){if($.id)c.updateUserPlaylist($.id,$.name,$.description,$.colorHex,$.iconName);else{const y=c.createUserPlaylist($.name,$.description,$.colorHex,$.iconName);z(r,{kind:"playlist",id:y.id},!0)}}function f($){c.deleteUserPlaylist($.id),n(r).kind==="playlist"&&n(r).id===$.id&&z(r,{kind:"overview"},!0)}var x=Ru(),g=K(x);{var k=$=>{Ti($,{mode:"favorites",onBack:()=>z(r,{kind:"overview"},!0)})},_=$=>{Ti($,{mode:"playlist",get playlist(){return n(o)},onBack:()=>z(r,{kind:"overview"},!0),onEdit:l,onDelete:f})},S=$=>{var y=Lu(),b=u(y),w=u(b),A=u(w),N=T(A,!0),R=d(A,2),p=T(R,!0),j=d(w,2),E=u(j);yr(E,{size:15});var C=d(E),q=d(b,2);ku(q,{onOpenFavorites:()=>z(r,{kind:"favorites"},!0),onOpenPlaylist:X=>z(r,{kind:"playlist",id:X},!0),onEditPlaylist:l});var U=d(q,2);Iu(U,{onCreatePlaylist:s}),O((X,B,W)=>{m(N,X),m(p,B),m(C,` ${W??""}`)},[()=>P("collection.title"),()=>P("collection.subtitle"),()=>P("collection.new_playlist")]),M("click",j,s),h($,y)};Q(g,$=>{n(r).kind==="favorites"?$(k):n(r).kind==="playlist"&&n(o)?$(_,1):$(S,-1)})}var I=d(g,2);mu(I,{get open(){return n(a)},get editing(){return n(i)},onClose:()=>z(a,!1),onSave:v}),h(t,x),ze()}Ve(["click"]);var qu=L('<div class="h-8 rounded-full bg-white/[0.07]"></div>'),Ou=L('<div class="flex items-center gap-3 px-3.5 py-2.5 border-b border-[var(--border)] last:border-b-0"><div class="w-11 h-11 rounded-lg bg-white/[0.08] shrink-0"></div> <div class="flex-1 space-y-2"><div class="h-3.5 w-44 rounded bg-white/[0.09]"></div> <div class="h-3 w-32 rounded bg-white/[0.06]"></div></div> <div class="h-3 w-9 rounded bg-white/[0.06]"></div></div>'),Du=L('<div class="rounded-2xl p-2.5 bg-[var(--surface)] border border-[var(--border)] space-y-2.5"><div class="aspect-square w-full rounded-xl bg-white/[0.08]"></div> <div class="h-3.5 w-4/5 rounded bg-white/[0.08]"></div> <div class="h-3 w-1/2 rounded bg-white/[0.05]"></div></div>'),Fu=L('<div class="space-y-6 animate-pulse" aria-busy="true" aria-label="Searching"><div class="flex items-center gap-2"></div> <div class="grid grid-cols-1 md:grid-cols-[minmax(240px,300px)_minmax(0,1fr)] gap-5"><div class="space-y-2.5"><div class="h-4 w-24 rounded-md bg-white/[0.07]"></div> <div class="rounded-2xl p-5 bg-[var(--surface)] border border-[var(--border)] space-y-4"><div class="w-20 h-20 rounded-full bg-white/[0.08]"></div> <div class="h-6 w-40 rounded-lg bg-white/[0.09]"></div> <div class="flex items-center justify-between pt-2"><div class="h-5 w-28 rounded-full bg-white/[0.07]"></div> <div class="w-10 h-10 rounded-full bg-white/[0.09]"></div></div></div></div> <div class="space-y-2.5"><div class="h-4 w-20 rounded-md bg-white/[0.07]"></div> <div class="track-list"></div></div></div> <div class="space-y-3"><div class="h-5 w-28 rounded-md bg-white/[0.08]"></div> <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-3"></div></div></div>'),Hu=L('<span class="muted"> </span>'),Vu=L('<section class="overview-col svelte-8hoooi"><h3 class="col-title svelte-8hoooi">Top result</h3> <div class="top-result-card svelte-8hoooi" role="button" tabindex="0"><!> <h2 class="top-name svelte-8hoooi"> </h2> <div class="top-bottom svelte-8hoooi"><div class="top-meta svelte-8hoooi"><span class="type-pill svelte-8hoooi">Artist</span> <!></div> <button class="play-circle small-play svelte-8hoooi" title="Play">▶</button></div></div></section>'),Qu=L('<button class="see-all svelte-8hoooi">See all →</button>'),zi=L('<div role="button" tabindex="0"><!> <div class="track-meta"><span class="track-title"> </span> <span class="track-sub"> </span></div> <span class="muted numeric dur svelte-8hoooi"> </span> <button title="Favorite"><!></button> <button class="icon-btn" title="Add to queue"><!></button></div>'),Uu=L('<section class="overview-col songs-col svelte-8hoooi"><div class="col-head svelte-8hoooi"><h3 class="col-title svelte-8hoooi">Songs</h3> <!></div> <div class="track-list"></div></section>'),Bu=L('<div class="overview-grid svelte-8hoooi"><!> <!></div>'),Wu=L('<span class="muted small svelte-8hoooi"> </span>'),Gu=L('<button class="artist-card svelte-8hoooi"><!> <strong> </strong> <!></button>'),dn=L('<div class="section svelte-8hoooi"><!></div>'),Ni=L('<button class="media-card svelte-8hoooi"><!> <strong class="media-title svelte-8hoooi"> </strong> <span class="media-sub muted svelte-8hoooi"> </span></button>'),Yu=L('<section class="section svelte-8hoooi"><h2 class="section-title">Songs</h2> <div class="track-list"></div></section>'),Ku=L('<div class="filters svelte-8hoooi"><button class="back-explore svelte-8hoooi"> </button> <button> </button> <button> <!></button> <button> <!></button> <button> <!></button> <button> <!></button></div> <!> <!> <!> <!> <!>',1),Ju=L('<button class="mood-tile svelte-8hoooi"><span class="mood-dot svelte-8hoooi">♪</span> <span class="genre-name svelte-8hoooi"> </span> <span class="muted">›</span></button>'),Mi=L('<section class="section svelte-8hoooi"><h2 class="section-title"> </h2> <div class="genre-grid svelte-8hoooi"></div></section>'),Xu=L('<button class="genre-chip svelte-8hoooi"><span class="genre-bar svelte-8hoooi"></span> <span class="genre-name svelte-8hoooi"> </span> <span class="muted">›</span></button>'),Zu=L('<h1 class="explore-title svelte-8hoooi"> </h1> <p class="muted caption svelte-8hoooi"> </p> <!> <!>',1),ev=L('<div class="page-clamp"><!></div>');function tv(t,e){Ae(e,!0);let r=G("all");const a=["#2190a4","#e66100","#3584e4","#9141ac","#2ec27e","#e01b24","#c061cb","#f5c211","#00b4d8","#625df5"],i=["chill","christmas","commute","energize","feel good","focus","gaming","party","romance","sad","sleep","workout"];function o(y){let b=2166136261;for(let w=0;w<y.length;w++)b^=y.toLowerCase().charCodeAt(w),b=Math.imul(b,16777619);return a[Math.abs(b)%a.length]}let s=J(()=>c.genres.filter(y=>i.some(b=>y.name.toLowerCase().includes(b)))),l=J(()=>c.genres.filter(y=>!i.some(b=>y.name.toLowerCase().includes(b)))),v=J(()=>c.searchResults),f=J(()=>n(v)?.artists[0]??null),x=J(()=>n(v)?.tracks.slice(0,4)??[]),g=J(()=>n(r)==="all"&&!!n(f)&&(n(v)?.artists.length??0)===1);var k=ev(),_=u(k);{var S=y=>{var b=Fu(),w=u(b);Re(w,20,()=>[64,92,96,86,98],Rt,(E,C)=>{var q=qu();O(()=>Ke(q,`width: ${C??""}px;`)),h(E,q)});var A=d(w,2),N=d(u(A),2),R=d(u(N),2);Re(R,20,()=>[0,1,2,3],Rt,(E,C)=>{var q=Ou();h(E,q)});var p=d(A,2),j=d(u(p),2);Re(j,20,()=>[0,1,2,3,4],Rt,(E,C)=>{var q=Du();h(E,q)}),h(y,b)},I=y=>{var b=Ku(),w=K(b),A=u(w),N=T(A,!0),R=d(A,2);let p;var j=T(R,!0),E=d(R,2);let C;var q=u(E),U=d(q);{var X=ye=>{var Pe=Or();O(()=>m(Pe,` · ${n(v).tracks.length??""}`)),h(ye,Pe)};Q(U,ye=>{n(v).tracks.length&&ye(X)})}var B=d(E,2);let W;var H=u(B),F=d(H);{var D=ye=>{var Pe=Or();O(()=>m(Pe,` · ${n(v).albums.length??""}`)),h(ye,Pe)};Q(F,ye=>{n(v).albums.length&&ye(D)})}var V=d(B,2);let Z;var Y=u(V),pe=d(Y);{var de=ye=>{var Pe=Or();O(()=>m(Pe,` · ${n(v).artists.length??""}`)),h(ye,Pe)};Q(pe,ye=>{n(v).artists.length&&ye(de)})}var oe=d(V,2);let re;var ae=u(oe),ge=d(ae);{var Te=ye=>{var Pe=Or();O(()=>m(Pe,` · ${n(v).playlists.length??""}`)),h(ye,Pe)};Q(ge,ye=>{n(v).playlists.length&&ye(Te)})}var ce=d(w,2);{var be=ye=>{var Pe=Bu(),ke=u(Pe);{var Me=Se=>{var Ce=Vu(),$e=d(u(Ce),2),Be=u($e);nt(Be,{class:"top-avatar",get src(){return n(f).avatar_url},get alt(){return n(f).name},fallbackIcon:"👤"});var tt=d(Be,2),Xe=T(tt,!0),st=d(tt,2),Fe=u(st),ht=d(u(Fe),2);{var dt=_e=>{var De=Hu(),te=T(De,!0);O(xe=>m(te,xe),[()=>Da(n(f).subscriber_count)]),h(_e,De)};Q(ht,_e=>{n(f).subscriber_count&&_e(dt)})}var ie=d(Fe,2);O(()=>m(Xe,n(f).name)),M("click",$e,()=>c.openArtist(n(f).id,n(f).name)),M("keydown",$e,_e=>_e.key==="Enter"&&c.openArtist(n(f).id,n(f).name)),M("click",ie,_e=>{_e.stopPropagation(),n(x)[0]&&c.replaceQueueAndPlay([n(x)[0]],0,`Search: ${c.searchQuery}`)}),h(Se,Ce)};Q(ke,Se=>{n(f)&&Se(Me)})}var Oe=d(ke,2);{var ne=Se=>{var Ce=Uu(),$e=u(Ce),Be=d(u($e),2);{var tt=st=>{var Fe=Qu();M("click",Fe,()=>z(r,"songs")),h(st,Fe)};Q(Be,st=>{n(v).tracks.length>4&&st(tt)})}var Xe=d($e,2);Re(Xe,21,()=>n(x),st=>st.id,(st,Fe)=>{const ht=J(()=>c.currentTrack?.id===n(Fe).id);var dt=zi();let ie;var _e=u(dt);nt(_e,{class:"track-cover",get src(){return n(Fe).cover_url},alt:""});var De=d(_e,2),te=u(De),xe=T(te,!0),Ie=d(te,2),Qe=T(Ie),Ze=d(De,2),xt=T(Ze,!0),mt=d(Ze,2);let Mt;var rt=u(mt);{let ct=J(()=>c.isFavorite(n(Fe).id)?"currentColor":"none");gt(rt,{size:16,get fill(){return n(ct)}})}var Pt=d(mt,2),yt=u(Pt);yr(yt,{size:16}),O((ct,fr)=>{ie=ee(dt,1,"track-row",null,ie,{active:n(ht)}),m(xe,n(Fe).title),m(Qe,`${n(Fe).artist??""}${n(Fe).album?` • ${n(Fe).album}`:""}`),m(xt,ct),Mt=ee(mt,1,"icon-btn",null,Mt,{"fav-active":fr})},[()=>zt(n(Fe).duration_secs),()=>c.isFavorite(n(Fe).id)]),M("click",dt,()=>c.replaceQueueAndPlay([n(Fe)],0,`Search: ${c.searchQuery}`)),M("keydown",dt,ct=>ct.key==="Enter"&&c.replaceQueueAndPlay([n(Fe)],0,`Search: ${c.searchQuery}`)),M("click",mt,ct=>{ct.stopPropagation(),c.toggleFavorite(n(Fe))}),M("click",Pt,ct=>{ct.stopPropagation(),c.enqueueTrack(n(Fe))}),h(st,dt)}),h(Se,Ce)};Q(Oe,Se=>{n(x).length>0&&Se(ne)})}h(ye,Pe)};Q(ce,ye=>{n(r)==="all"&&(n(f)||n(x).length>0)&&ye(be)})}var me=d(ce,2);{var Ee=ye=>{var Pe=dn(),ke=u(Pe);Er(ke,{title:"Artists",minCardWidth:144,children:(Me,Oe)=>{var ne=se(),Se=K(ne);Re(Se,17,()=>n(v).artists,Ce=>Ce.id,(Ce,$e)=>{var Be=Gu(),tt=u(Be);nt(tt,{class:"artist-avatar",get src(){return n($e).avatar_url},get alt(){return n($e).name},fallbackIcon:"👤"});var Xe=d(tt,2),st=T(Xe,!0),Fe=d(Xe,2);{var ht=dt=>{var ie=Wu(),_e=T(ie,!0);O(De=>m(_e,De),[()=>Da(n($e).subscriber_count)]),h(dt,ie)};Q(Fe,dt=>{n($e).subscriber_count&&dt(ht)})}O(()=>m(st,n($e).name)),M("click",Be,()=>c.openArtist(n($e).id,n($e).name)),h(Ce,Be)}),h(Me,ne)},$$slots:{default:!0}}),h(ye,Pe)};Q(me,ye=>{(n(r)==="all"||n(r)==="artists")&&n(v).artists.length>0&&!n(g)&&ye(Ee)})}var we=d(me,2);{var qe=ye=>{var Pe=dn(),ke=u(Pe);Er(ke,{title:"Albums",children:(Me,Oe)=>{var ne=se(),Se=K(ne);Re(Se,17,()=>n(v).albums,Ce=>Ce.id,(Ce,$e)=>{var Be=Ni(),tt=u(Be);nt(tt,{class:"media-cover",get src(){return n($e).cover_url},get alt(){return n($e).title}});var Xe=d(tt,2),st=T(Xe,!0),Fe=d(Xe,2),ht=T(Fe,!0);O(()=>{m(st,n($e).title),m(ht,n($e).artist)}),M("click",Be,()=>c.openAlbum(n($e).id,n($e).title,n($e).cover_url)),h(Ce,Be)}),h(Me,ne)},$$slots:{default:!0}}),h(ye,Pe)};Q(we,ye=>{(n(r)==="all"||n(r)==="albums")&&n(v).albums.length>0&&ye(qe)})}var We=d(we,2);{var Ye=ye=>{var Pe=dn(),ke=u(Pe);Er(ke,{title:"Playlists",children:(Me,Oe)=>{var ne=se(),Se=K(ne);Re(Se,17,()=>n(v).playlists,Ce=>Ce.id,(Ce,$e)=>{var Be=Ni(),tt=u(Be);nt(tt,{class:"media-cover",get src(){return n($e).cover_url},get alt(){return n($e).name}});var Xe=d(tt,2),st=T(Xe,!0),Fe=d(Xe,2),ht=T(Fe,!0);O(()=>{m(st,n($e).name),m(ht,n($e).subtitle)}),M("click",Be,()=>c.openAlbum(n($e).id,n($e).name,n($e).cover_url,!0)),h(Ce,Be)}),h(Me,ne)},$$slots:{default:!0}}),h(ye,Pe)};Q(We,ye=>{(n(r)==="all"||n(r)==="playlists")&&n(v).playlists.length>0&&ye(Ye)})}var vt=d(We,2);{var lt=ye=>{var Pe=Yu(),ke=d(u(Pe),2);Re(ke,21,()=>n(v).tracks,Me=>Me.id,(Me,Oe)=>{const ne=J(()=>c.currentTrack?.id===n(Oe).id);var Se=zi();let Ce;var $e=u(Se);nt($e,{class:"track-cover",get src(){return n(Oe).cover_url},alt:""});var Be=d($e,2),tt=u(Be),Xe=T(tt,!0),st=d(tt,2),Fe=T(st),ht=d(Be,2),dt=T(ht,!0),ie=d(ht,2);let _e;var De=u(ie);{let Ie=J(()=>c.isFavorite(n(Oe).id)?"currentColor":"none");gt(De,{size:16,get fill(){return n(Ie)}})}var te=d(ie,2),xe=u(te);yr(xe,{size:16}),O((Ie,Qe)=>{Ce=ee(Se,1,"track-row",null,Ce,{active:n(ne)}),m(Xe,n(Oe).title),m(Fe,`${n(Oe).artist??""}${n(Oe).album?` • ${n(Oe).album}`:""}`),m(dt,Ie),_e=ee(ie,1,"icon-btn",null,_e,{"fav-active":Qe})},[()=>zt(n(Oe).duration_secs),()=>c.isFavorite(n(Oe).id)]),M("click",Se,()=>c.replaceQueueAndPlay([n(Oe)],0,`Search: ${c.searchQuery}`)),M("keydown",Se,Ie=>Ie.key==="Enter"&&c.replaceQueueAndPlay([n(Oe)],0,`Search: ${c.searchQuery}`)),M("click",ie,Ie=>{Ie.stopPropagation(),c.toggleFavorite(n(Oe))}),M("click",te,Ie=>{Ie.stopPropagation(),c.enqueueTrack(n(Oe))}),h(Me,Se)}),h(ye,Pe)};Q(vt,ye=>{n(r)==="songs"&&n(v).tracks.length>0&&ye(lt)})}O((ye,Pe,ke,Me,Oe,ne,Se)=>{le(A,"title",ye),m(N,Pe),p=ee(R,1,"svelte-8hoooi",null,p,{active:n(r)==="all"}),m(j,ke),C=ee(E,1,"svelte-8hoooi",null,C,{active:n(r)==="songs"}),m(q,Me),W=ee(B,1,"svelte-8hoooi",null,W,{active:n(r)==="albums"}),m(H,Oe),Z=ee(V,1,"svelte-8hoooi",null,Z,{active:n(r)==="artists"}),m(Y,ne),re=ee(oe,1,"svelte-8hoooi",null,re,{active:n(r)==="playlists"}),m(ae,Se)},[()=>P("explore.back"),()=>P("explore.back"),()=>P("explore.all"),()=>P("explore.songs"),()=>P("explore.albums"),()=>P("explore.artists"),()=>P("explore.playlists")]),M("click",A,()=>c.clearSearch()),M("click",R,()=>z(r,"all")),M("click",E,()=>z(r,"songs")),M("click",B,()=>z(r,"albums")),M("click",V,()=>z(r,"artists")),M("click",oe,()=>z(r,"playlists")),h(y,b)},$=y=>{var b=Zu(),w=K(b),A=T(w,!0),N=d(w,2),R=T(N,!0),p=d(N,2);{var j=q=>{var U=Mi(),X=u(U),B=T(X,!0),W=d(X,2);Re(W,21,()=>n(s),H=>H.id,(H,F)=>{const D=J(()=>o(n(F).name));var V=Ju(),Z=u(V),Y=d(Z,2),pe=T(Y,!0);O(()=>{Ke(V,`background: linear-gradient(135deg, ${n(D)??""}33, var(--surface) 68%); border-color: ${n(D)??""}44;`),Ke(Z,`background: ${n(D)??""}33; color: ${n(D)??""};`),m(pe,n(F).name)}),M("click",V,()=>void c.runSearch(n(F).name)),h(H,V)}),O(H=>m(B,H),[()=>P("explore.moods")]),h(q,U)};Q(p,q=>{n(s).length>0&&q(j)})}var E=d(p,2);{var C=q=>{var U=Mi(),X=u(U),B=T(X,!0),W=d(X,2);Re(W,21,()=>n(l),H=>H.id,(H,F)=>{const D=J(()=>o(n(F).name));var V=Xu(),Z=u(V),Y=d(Z,2),pe=T(Y,!0);O(()=>{Ke(V,`background: linear-gradient(90deg, ${n(D)??""}26, var(--surface) 62%);`),Ke(Z,`background: ${n(D)??""};`),m(pe,n(F).name)}),M("click",V,()=>void c.runSearch(n(F).name)),h(H,V)}),O(H=>m(B,H),[()=>P("explore.genres")]),h(q,U)};Q(E,q=>{n(l).length>0&&q(C)})}O((q,U)=>{m(A,q),m(R,U)},[()=>P("explore.title"),()=>P("explore.subtitle")]),h(y,b)};Q(_,y=>{c.searchLoading?y(S):n(v)?y(I,1):y($,-1)})}h(t,k),ze()}Ve(["click","keydown"]);var rv=L('<span class="w-2 h-2 rounded-full bg-emerald-400 ring-2 ring-[var(--bg-elevated)] absolute top-1 right-1 animate-pulse"></span>'),av=L('<div class="justify-self-end flex items-center gap-0.5 sm:gap-1.5 shrink-0"><button type="button"><!> <!></button> <button type="button"><!></button> <span class="hidden sm:block w-px h-5 bg-[var(--border-strong)] mx-1" aria-hidden="true"></span> <button type="button"><!></button> <button type="button"><!></button></div>');function nv(t,e){Ae(e,!0);let r=J(()=>c.spinStatus.local_active||c.spinStatus.remote_role!=="idle");var a=av(),i=u(a);let o;var s=u(i);$a(s,{size:18});var l=d(s,2);{var v=b=>{var w=rv();h(b,w)};Q(l,b=>{n(r)&&b(v)})}var f=d(i,2);let x;var g=u(f);Bn(g,{size:18});var k=d(f,4);let _;var S=u(k);Vc(S,{size:18});var I=d(k,2);let $;var y=u(I);Fc(y,{size:18}),O((b,w,A,N)=>{o=ee(i,1,"icon-btn relative",null,o,{active:c.spinOpen||n(r)}),le(i,"title",b),x=ee(f,1,"icon-btn hidden sm:inline-flex",null,x,{active:c.partyMode}),le(f,"title",w),_=ee(k,1,"icon-btn",null,_,{active:c.settingsOpen}),le(k,"title",A),$=ee(I,1,"icon-btn hidden md:inline-flex",null,$,{active:c.sidebarOpen}),le(I,"title",N)},[()=>P("header.spin_mode"),()=>P("header.party_mode"),()=>P("header.settings"),()=>P("header.toggle_sidebar")]),M("click",i,()=>{c.spinOpen=!c.spinOpen,c.spinOpen&&c.refreshSpinStatus()}),M("click",f,()=>c.partyMode=!c.partyMode),M("click",k,()=>c.settingsOpen=!0),M("click",I,()=>{c.sidebarOpen=!c.sidebarOpen,c.persistSession()}),h(t,a),ze()}Ve(["click"]);var la=pc(()=>c),iv=L('<div class="justify-self-start flex items-center gap-2 min-w-0"><button type="button"><!></button> <button type="button" class="flex items-center gap-2 rounded-xl px-1.5 py-1 hover:bg-[var(--surface)] transition-colors active:scale-[0.98]"><img src="/gyrotube.svg" alt="" class="w-6 h-6 shrink-0 drop-shadow-[0_2px_8px_rgba(33,144,164,0.45)]"/> <span class="font-extrabold text-[15.5px] tracking-[-0.02em] hidden sm:inline">GyroTube</span> <span class="hidden md:inline-flex items-center px-2 py-0.5 rounded-full text-[10.5px] font-bold tracking-wide bg-[var(--accent-soft)] text-[var(--accent-bright)] border border-[var(--accent-border)]/40"> </span></button></div>');function sv(t,e){Ae(e,!1),Qn();var r=iv(),a=u(r),i=u(a);Dc(i,{size:18});var o=d(a,2),s=d(u(o),4),l=T(s,!0);O((v,f,x)=>{ee(a,1,`icon-btn shrink-0 hidden md:inline-flex ${la().navCollapsed?"text-[var(--text-muted)]":"text-[var(--text)]"}`),le(a,"title",v),le(o,"title",f),m(l,x)},[()=>P("header.toggle_sidebar"),()=>P("header.go_home"),()=>P("brand.music")]),M("click",a,()=>la(la().navCollapsed=!la().navCollapsed)),M("click",o,()=>la().navigateTab("home")),h(t,r),ze()}Ve(["click"]);var ov=L('<button type="button" class="w-5 h-5 rounded-full inline-flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text)] hover:bg-[var(--surface-hover)] shrink-0 transition-colors"><!></button>'),lv=L('<kbd class="hidden sm:inline-flex items-center gap-0.5 px-1.5 py-0.5 rounded-[6px] text-[10.5px] font-mono font-semibold text-[var(--text-muted)] bg-white/[0.06] border border-white/10 shrink-0 leading-none">Ctrl K</kbd>'),cv=L('<button type="button" class="text-[11.5px] text-[var(--accent-bright)] hover:text-[var(--text)] normal-case font-semibold transition-colors"> </button>'),dv=L('<span class="text-[var(--text-muted)]"> </span><strong class="font-semibold text-[var(--text)]"> </strong>',1),Ei=L('<button type="button" class="w-7 h-7 mr-1 rounded-lg inline-flex items-center justify-center text-[var(--text-muted)] opacity-0 group-hover:opacity-100 hover:text-[var(--text)] hover:bg-white/[0.1] transition-all shrink-0"><!></button>'),uv=L('<div role="option"><button type="button" class="flex-1 min-w-0 flex items-center gap-2.5 px-2.5 py-2 text-left text-[13.5px]"><!> <span class="truncate"><!></span></button> <!></div>'),vv=L('<div class="absolute left-0 right-0 top-[calc(100%+8px)] rounded-2xl bg-[var(--bg-elevated)]/98 backdrop-blur-xl border border-[var(--border-strong)] shadow-[0_20px_48px_rgba(0,0,0,0.55)] overflow-hidden py-1.5 z-50" role="listbox"><div class="flex items-center justify-between px-4 pt-1.5 pb-2 mb-1 text-[10.5px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)] border-b border-white/[0.06]"><span> </span> <!></div> <div class="px-1.5 space-y-0.5 max-h-[320px] overflow-y-auto scroll-y"></div></div>'),fv=L('<div class="w-full relative"><div class="flex items-center gap-2.5 h-9 px-3.5 rounded-full bg-[var(--surface)] border border-[var(--border)] hover:border-[var(--border-strong)] focus-within:border-[var(--accent-border)] focus-within:bg-[var(--bg)] focus-within:ring-2 focus-within:ring-[var(--accent-soft)] transition-all"><!> <input type="text" class="w-full bg-transparent outline-none text-[13.5px] placeholder:text-[var(--text-muted)]"/> <!></div> <!></div>');function hv(t,e){Ae(e,!0);let r=G(""),a=G(Je([])),i=G(!1),o=G(-1),s,l=G(null),v=J(()=>n(r).trim().length===0),f=J(()=>n(v)?c.searchHistory:n(a));$t(()=>{!c.searchQuery&&n(r)&&document.activeElement!==n(l)&&z(r,"")}),Sa(()=>{const E=C=>{(C.ctrlKey||C.metaKey)&&C.key.toLowerCase()==="k"&&(C.preventDefault(),n(l)?.focus(),n(l)?.select())};return window.addEventListener("keydown",E),()=>window.removeEventListener("keydown",E)});function x(){z(o,-1),window.clearTimeout(s);const E=n(r).trim();if(!E){z(a,[],!0),c.clearSearch(),z(i,c.searchHistory.length>0);return}z(i,!0),s=window.setTimeout(async()=>{if(Ue())try{z(a,await Ne.getSearchSuggestions(E),!0),z(i,n(a).length>0)}catch{z(a,[],!0)}},180)}function g(E){const C=E.trim();C&&(z(r,C,!0),z(i,!1),n(l)?.blur(),c.runSearch(C))}function k(E){z(r,E,!0),z(o,-1),n(l)?.focus(),x()}function _(){z(r,""),z(a,[],!0),z(o,-1),c.clearSearch(),z(i,c.searchHistory.length>0),n(l)?.focus()}function S(E){if(!n(i)||n(f).length===0){E.key==="Enter"?g(n(r)):E.key==="Escape"&&n(l)?.blur();return}E.key==="ArrowDown"?(E.preventDefault(),z(o,(n(o)+1)%n(f).length)):E.key==="ArrowUp"?(E.preventDefault(),z(o,(n(o)-1+n(f).length)%n(f).length)):E.key==="Enter"?(E.preventDefault(),g(n(o)>=0?n(f)[n(o)]:n(r))):E.key==="Escape"&&z(i,!1)}function I(E){const C=n(r).trim();return!n(v)&&C&&E.toLowerCase().startsWith(C.toLowerCase())?[E.slice(0,C.length),E.slice(C.length)]:["",E]}var $=fv(),y=u($),b=u(y);xi(b,{size:15,class:"text-[var(--text-muted)] shrink-0"});var w=d(b,2);Xr(w,E=>z(l,E),()=>n(l));var A=d(w,2);{var N=E=>{var C=ov(),q=u(C);Cr(q,{size:13}),O(U=>le(C,"title",U),[()=>P("header.clear_search")]),M("mousedown",C,U=>{U.preventDefault(),_()}),h(E,C)},R=E=>{var C=lv();h(E,C)};Q(A,E=>{n(r).length>0?E(N):E(R,-1)})}var p=d(y,2);{var j=E=>{var C=vv(),q=u(C),U=u(q),X=T(U,!0),B=d(U,2);{var W=F=>{var D=cv(),V=T(D,!0);O(Z=>m(V,Z),[()=>P("header.clear")]),M("mousedown",D,Z=>{Z.preventDefault(),c.clearSearchHistory(),z(i,!1)}),h(F,D)};Q(B,F=>{n(v)&&F(W)})}var H=d(q,2);Re(H,22,()=>n(f),F=>F,(F,D,V)=>{const Z=J(()=>{const[we,qe]=I(D);return{prefix:we,rest:qe}});var Y=uv(),pe=u(Y),de=u(pe);{var oe=we=>{Ac(we,{size:14,class:"text-[var(--text-muted)] shrink-0"})},re=we=>{xi(we,{size:14,class:"text-[var(--text-muted)] shrink-0"})};Q(de,we=>{n(v)?we(oe):we(re,-1)})}var ae=d(de,2),ge=u(ae);{var Te=we=>{var qe=dv(),We=K(qe),Ye=T(We,!0),vt=d(We),lt=T(vt,!0);O(()=>{m(Ye,n(Z).prefix),m(lt,n(Z).rest)}),h(we,qe)},ce=we=>{var qe=Or();O(()=>m(qe,D)),h(we,qe)};Q(ge,we=>{n(Z).prefix?we(Te):we(ce,-1)})}var be=d(pe,2);{var me=we=>{var qe=Ei(),We=u(qe);Cr(We,{size:13}),O(Ye=>le(qe,"title",Ye),[()=>P("queue.remove")]),M("mousedown",qe,Ye=>{Ye.preventDefault(),c.removeSearchHistoryItem(D),c.searchHistory.length===0&&z(i,!1)}),h(we,qe)},Ee=we=>{var qe=Ei(),We=u(qe);Pc(We,{size:14}),O(()=>le(qe,"title",D)),M("mousedown",qe,Ye=>{Ye.preventDefault(),k(D)}),h(we,qe)};Q(be,we=>{n(v)?we(me):we(Ee,-1)})}O(()=>{ee(Y,1,`group flex items-center gap-1 rounded-xl transition-colors ${n(V)===n(o)?"bg-[var(--accent-soft)] text-[var(--accent-bright)]":"hover:bg-white/[0.06]"}`),le(Y,"aria-selected",n(V)===n(o))}),M("mousedown",pe,we=>{we.preventDefault(),g(D)}),h(F,Y)}),O(F=>m(X,F),[()=>n(v)?P("header.recent_searches"):P("header.suggestions")]),ot(3,C,()=>pr,()=>({y:-6,duration:160,easing:At})),h(E,C)};Q(p,E=>{n(i)&&n(f).length>0&&E(j)})}O(E=>le(w,"placeholder",E),[()=>P("header.search_placeholder")]),_t("focus",w,()=>{n(f).length>0&&z(i,!0)}),_t("blur",w,()=>window.setTimeout(()=>z(i,!1),150)),M("input",w,x),M("keydown",w,S),qa(w,()=>n(r),E=>z(r,E)),h(t,$),ze()}Ve(["input","keydown","mousedown"]);var pv=L('<header class="h-14 shrink-0 z-30 flex md:grid md:grid-cols-[1fr_minmax(240px,500px)_1fr] items-center gap-2 md:gap-3 px-3 md:px-4 bg-[var(--bg-elevated)]/95 backdrop-blur-md border-b border-[var(--border)] select-none"><!> <div class="flex-1 min-w-0 md:w-full"><!></div> <!></header>');function _v(t){var e=pv(),r=u(e);sv(r,{});var a=d(r,2),i=u(a);hv(i,{});var o=d(a,2);nv(o,{}),h(t,e)}var gv=L('<section class="mt-6"><div class="flex flex-col sm:flex-row items-start sm:items-center gap-6 p-5 sm:px-6 rounded-[18px] border border-[var(--border)] shadow-[0_12px_32px_rgba(0,0,0,0.28)]" style="background: linear-gradient(135deg, rgba(var(--ambient-rgb), 0.24), var(--surface) 65%);"><!> <div class="min-w-0 flex flex-col gap-1.5"><span class="self-start text-[11px] font-extrabold tracking-[0.08em] px-2.5 py-0.5 rounded-full bg-[var(--accent-soft)] text-[var(--accent-bright)]"> </span> <h2 class="text-[24px] font-extrabold tracking-tight leading-tight"> </h2> <p class="text-[14px] text-[var(--text-muted)]"> </p> <div class="flex items-center gap-2.5 mt-1.5"><button class="pill-btn primary"><!> </button> <button class="pill-btn"> </button></div></div></div></section>');function bv(t,e){Ae(e,!0);var r=gv(),a=u(r),i=u(a);nt(i,{class:"hero-cover",get src(){return e.album.cover_url},get alt(){return e.album.title}});var o=d(i,2),s=u(o),l=T(s,!0),v=d(s,2),f=T(v,!0),x=d(v,2),g=T(x,!0),k=d(x,2),_=u(k),S=u(_);bt(S,{size:14,fill:"currentColor"});var I=d(S),$=d(_,2),y=T($,!0);O((b,w,A)=>{m(l,b),m(f,e.album.title),m(g,e.album.artist),m(I,` ${w??""}`),m(y,A)},[()=>P("home.featured"),()=>P("home.play"),()=>P("home.open_album")]),M("click",_,()=>c.openAlbum(e.album.id,e.album.title,e.album.cover_url)),M("click",$,()=>c.openAlbum(e.album.id,e.album.title,e.album.cover_url)),h(t,r),ze()}Ve(["click"]);var xv=L('<button><!> <div class="track-meta"><span> </span> <span class="track-sub"> </span></div> <span aria-hidden="true"><!></span></button>'),mv=L('<section class="mt-6 flex flex-col gap-3"><h2 class="section-title"> </h2> <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-2.5"></div></section>');function yv(t,e){Ae(e,!0);var r=se(),a=K(r);{var i=o=>{var s=mv(),l=u(s),v=T(l,!0),f=d(l,2);Re(f,23,()=>e.tracks,x=>x.id,(x,g,k)=>{const _=J(()=>c.currentTrack?.id===n(g).id);var S=xv(),I=u(S);nt(I,{class:"tile-cover",get src(){return n(g).cover_url},alt:""});var $=d(I,2),y=u($),b=T(y,!0),w=d(y,2),A=T(w,!0),N=d($,2),R=u(N);{var p=E=>{vr(E,{size:14})},j=E=>{bt(E,{size:13,fill:"currentColor",class:"translate-x-[1px]"})};Q(R,E=>{n(_)?E(p):E(j,-1)})}O(()=>{ee(S,1,`group flex items-center gap-3 p-2 pr-3 rounded-xl border text-left transition-all duration-150 ${n(_)?"bg-[var(--accent-soft)] border-[var(--accent-border)]":"bg-[var(--surface)] border-[var(--border)] hover:bg-[var(--surface-hover)]"}`),ee(y,1,`track-title ${n(_)?"!text-[var(--accent-bright)] !font-bold":""}`),m(b,n(g).title),m(A,n(g).artist),ee(N,1,`w-7 h-7 rounded-full inline-flex items-center justify-center shrink-0 transition-all duration-150 ${n(_)?"bg-[var(--accent-soft)] text-[var(--accent-bright)]":"text-[var(--text-muted)] group-hover:bg-[var(--accent)] group-hover:text-white group-hover:shadow-[0_4px_12px_rgba(33,144,164,0.42)]"}`)}),M("click",S,()=>c.replaceQueueAndPlay(e.tracks,n(k),"Listen again")),h(x,S)}),O(x=>m(v,x),[()=>P("home.listen_again")]),h(o,s)};Q(a,o=>{e.tracks.length>0&&o(i)})}h(t,r),ze()}Ve(["click"]);var wv=L('<section class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-2.5 mb-6"><button class="flex items-center gap-3 px-3.5 py-3 rounded-2xl border border-[var(--border)] hover:border-[var(--border-strong)] text-left transition-all duration-150 hover:-translate-y-0.5 active:scale-[0.98]" style="background: linear-gradient(135deg, rgba(224, 27, 36, 0.16), var(--surface) 70%);"><span class="w-9 h-9 rounded-xl inline-flex items-center justify-center bg-[var(--surface-hover)] shrink-0"><!></span> <span class="min-w-0 flex flex-col gap-0.5"><strong class="text-[14px] truncate"> </strong> <small class="text-[11.5px] text-[var(--text-muted)] truncate"> </small></span></button> <button class="flex items-center gap-3 px-3.5 py-3 rounded-2xl border border-[var(--border)] hover:border-[var(--border-strong)] text-left transition-all duration-150 hover:-translate-y-0.5 active:scale-[0.98]" style="background: linear-gradient(135deg, rgba(53, 132, 228, 0.16), var(--surface) 70%);"><span class="w-9 h-9 rounded-xl inline-flex items-center justify-center bg-[var(--surface-hover)] shrink-0"><!></span> <span class="min-w-0 flex flex-col gap-0.5"><strong class="text-[14px] truncate"> </strong> <small class="text-[11.5px] text-[var(--text-muted)] truncate"> </small></span></button> <button class="flex items-center gap-3 px-3.5 py-3 rounded-2xl border border-[var(--border)] hover:border-[var(--border-strong)] text-left transition-all duration-150 hover:-translate-y-0.5 active:scale-[0.98]" style="background: linear-gradient(135deg, rgba(192, 97, 203, 0.16), var(--surface) 70%);"><span class="w-9 h-9 rounded-xl inline-flex items-center justify-center bg-[var(--surface-hover)] shrink-0"><!></span> <span class="min-w-0 flex flex-col gap-0.5"><strong class="text-[14px] truncate"> </strong> <small class="text-[11.5px] text-[var(--text-muted)] truncate"> </small></span></button> <button class="flex items-center gap-3 px-3.5 py-3 rounded-2xl border border-[var(--border)] hover:border-[var(--border-strong)] text-left transition-all duration-150 hover:-translate-y-0.5 active:scale-[0.98]" style="background: linear-gradient(135deg, rgba(46, 194, 126, 0.16), var(--surface) 70%);"><span class="w-9 h-9 rounded-xl inline-flex items-center justify-center bg-[var(--surface-hover)] shrink-0"><!></span> <span class="min-w-0 flex flex-col gap-0.5"><strong class="text-[14px] truncate"> </strong> <small class="text-[11.5px] text-[var(--text-muted)] truncate"> </small></span></button></section>');function kv(t,e){Ae(e,!1),Qn();var r=wv(),a=u(r),i=u(a),o=u(i);Us(o,{size:18});var s=d(i,2),l=u(s),v=T(l,!0),f=d(l,2),x=T(f,!0),g=d(a,2),k=u(g),_=u(k);Gc(_,{size:18});var S=d(k,2),I=u(S),$=T(I,!0),y=d(I,2),b=T(y,!0),w=d(g,2),A=u(w),N=u(A);xa(N,{size:18});var R=d(A,2),p=u(R),j=T(p,!0),E=d(p,2),C=T(E,!0),q=d(w,2),U=u(q),X=u(U);gt(X,{size:18});var B=d(U,2),W=u(B),H=T(W,!0),F=d(W,2),D=T(F,!0);O((V,Z,Y,pe,de,oe,re,ae)=>{m(v,V),m(x,Z),m($,Y),m(b,pe),m(j,de),m(C,oe),m(H,re),m(D,ae)},[()=>P("home.new_releases"),()=>P("home.new_releases_sub"),()=>P("home.top_charts"),()=>P("home.top_charts_sub"),()=>P("home.moods"),()=>P("home.moods_sub"),()=>P("home.collection"),()=>P("home.collection_sub")]),M("click",a,()=>document.getElementById("new-releases")?.scrollIntoView({behavior:"smooth"})),M("click",g,()=>document.getElementById("trending-hits")?.scrollIntoView({behavior:"smooth"})),M("click",w,()=>{c.clearSearch(),c.navigateTab("explore")}),M("click",q,()=>c.navigateTab("collection")),h(t,r),ze()}Ve(["click"]);var Sv=L('<img alt="" loading="lazy" decoding="async" referrerpolicy="no-referrer"/>'),$v=L('<span class="w-4 h-4 rounded-full border-2 border-white/30 border-t-white animate-spin"></span>'),Pv=L('<div class="group relative overflow-hidden rounded-2xl p-4 border border-white/10 hover:border-white/20 cursor-pointer transition-all duration-200 hover:-translate-y-0.5 flex flex-col justify-between min-h-[152px] bg-[var(--surface)]" role="button" tabindex="0"><!> <div class="absolute inset-0 pointer-events-none"></div> <div class="relative z-10 flex items-center justify-between gap-2"><span class="px-2.5 py-0.5 rounded-full text-[11px] font-bold bg-black/40 backdrop-blur-sm"> </span> <span class="opacity-90"><!></span></div> <div class="relative z-10 my-3"><h3 class="text-[18px] font-extrabold tracking-tight leading-snug drop-shadow-sm"> </h3> <p class="text-[12px] text-white/75 mt-0.5 line-clamp-1"> </p></div> <div class="relative z-10 flex items-center justify-between gap-2"><span class="inline-flex items-center gap-1 text-[12px] font-bold group-hover:translate-x-0.5 transition-transform"> <!></span> <button class="w-9 h-9 rounded-full inline-flex items-center justify-center bg-black/45 hover:bg-black/65 border border-white/15 text-white shadow-md transition-all active:scale-95"><!></button></div></div>'),Tv=L('<section class="mt-6 flex flex-col gap-3"><div class="flex items-center justify-between gap-3"><h2 class="section-title"> </h2> <button class="text-[12.5px] font-semibold text-[var(--accent-bright)] hover:underline"> </button></div> <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3.5"></div></section>');function Av(t,e){Ae(e,!0);const r=["#e05263","#2ec27e","#9141ac","#c061cb","#e66100","#2190a4"];let a=G(null),i=G(null),o=G(Je({})),s=G(Je({})),l=G(Je({})),v=new Set,f=G(!1);function x(y){return y.includes("googleusercontent.com")&&/=w\d+-h\d+/.test(y)?y.replace(/=w\d+-h\d+[^&?]*/,"=w360-h360-l90-rj"):y}Sa(()=>{if(!n(a)||typeof IntersectionObserver>"u"){z(f,!0);return}const y=new IntersectionObserver(b=>{b.some(w=>w.isIntersecting)&&(z(f,!0),y.disconnect())},{rootMargin:"220px"});return y.observe(n(a)),()=>y.disconnect()}),$t(()=>{if(!(!n(f)||!Ue()||e.stations.length===0))for(const y of e.stations)v.has(y.id)||(v.add(y.id),Ne.getGenrePlaylists(y.id).then(([b,w])=>{const A=w.find(N=>N.cover_url)??w[0];A&&(z(s,{...n(s),[y.id]:A.id},!0),A.cover_url&&z(o,{...n(o),[y.id]:x(A.cover_url)},!0))}).catch(()=>{}))});function g(y,b){return y.color_hex&&y.color_hex.startsWith("#")&&y.color_hex.length>=7?y.color_hex:r[b%r.length]}function k(y){const b=y.toLowerCase();return b.includes("chill")||b.includes("sleep")||b.includes("relax")?"moon":b.includes("commute")||b.includes("drive")||b.includes("travel")?"car":b.includes("energ")||b.includes("workout")||b.includes("party")?"zap":b.includes("feel")||b.includes("roman")||b.includes("love")?"heart":b.includes("focus")||b.includes("study")?"target":b.includes("christmas")||b.includes("holiday")||b.includes("mix")?"headphones":"music"}async function _(y){if(!(!Ue()||n(i))){z(i,y.id,!0);try{let b=n(s)[y.id];if(!b){const[w,A]=await Ne.getGenrePlaylists(y.id);b=A[0]?.id}if(b){const[w,A]=await Ne.getPlaylistTracks(b);if(A.length>0){c.replaceQueueAndPlay(A,0,w||y.name);return}}await c.runSearch(y.name)}catch(b){c.showToast(String(b))}finally{z(i,null)}}}var S=se(),I=K(S);{var $=y=>{var b=Tv(),w=u(b),A=u(w),N=T(A,!0),R=d(A,2),p=T(R,!0),j=d(w,2);Re(j,23,()=>e.stations,E=>E.id,(E,C,q)=>{const U=J(()=>g(n(C),n(q))),X=J(()=>k(n(C).name)),B=J(()=>n(o)[n(C).id]);var W=Pv(),H=u(W);{var F=ne=>{var Se=Sv();O(()=>{le(Se,"src",n(B)),ee(Se,1,`absolute inset-0 w-full h-full object-cover scale-105 group-hover:scale-110 transition-all duration-500 pointer-events-none ${n(l)[n(C).id]?"opacity-35":"opacity-0"}`)}),_t("load",Se,()=>z(l,{...n(l),[n(C).id]:!0},!0)),h(ne,Se)};Q(H,ne=>{n(B)&&ne(F)})}var D=d(H,2),V=d(D,2),Z=u(V),Y=T(Z,!0),pe=d(Z,2),de=u(pe);{var oe=ne=>{Un(ne,{size:16})},re=ne=>{Vs(ne,{size:16})},ae=ne=>{Zs(ne,{size:16})},ge=ne=>{gt(ne,{size:16,fill:"currentColor"})},Te=ne=>{Bc(ne,{size:16})},ce=ne=>{Ws(ne,{size:16})},be=ne=>{ea(ne,{size:16})};Q(de,ne=>{n(X)==="moon"?ne(oe):n(X)==="car"?ne(re,1):n(X)==="zap"?ne(ae,2):n(X)==="heart"?ne(ge,3):n(X)==="target"?ne(Te,4):n(X)==="headphones"?ne(ce,5):ne(be,-1)})}var me=d(V,2),Ee=u(me),we=T(Ee,!0),qe=d(Ee,2),We=T(qe,!0),Ye=d(me,2),vt=u(Ye),lt=u(vt),ye=d(lt);Sc(ye,{size:13});var Pe=d(vt,2),ke=u(Pe);{var Me=ne=>{var Se=$v();h(ne,Se)},Oe=ne=>{bt(ne,{size:15,fill:"currentColor",class:"translate-x-[1px]"})};Q(ke,ne=>{n(i)===n(C).id?ne(Me):ne(Oe,-1)})}O((ne,Se,Ce)=>{Ke(D,`background: linear-gradient(155deg, ${n(U)??""}80 0%, ${n(U)??""}44 45%, rgba(18, 18, 22, 0.88) 100%);`),Ke(Z,`color: ${n(U)??""};`),m(Y,ne),Ke(pe,`color: ${n(U)??""};`),m(we,n(C).name),m(We,Se),Ke(vt,`color: ${n(U)??""};`),m(lt,`${Ce??""} `),le(Pe,"title",`Play ${n(C).name??""}`),Pe.disabled=n(i)===n(C).id},[()=>P("home.station_badge"),()=>P("home.station_sub"),()=>P("home.explore_station")]),M("click",W,()=>void c.runSearch(n(C).name)),M("keydown",W,ne=>ne.key==="Enter"&&void c.runSearch(n(C).name)),M("click",Pe,ne=>{ne.stopPropagation(),_(n(C))}),h(E,W)}),Xr(b,E=>z(a,E),()=>n(a)),O((E,C)=>{m(N,E),m(p,C)},[()=>P("home.stations_title"),()=>P("home.see_all")]),M("click",R,()=>{c.clearSearch(),c.navigateTab("explore")}),h(y,b)};Q(I,y=>{e.stations.length>0&&y($)})}h(t,S),ze()}Ve(["click","keydown"]);var zv=L('<div role="button" tabindex="0"><span class="w-6 text-center text-[12px] font-bold text-[var(--text-muted)] numeric shrink-0"><!></span> <!> <div class="track-meta"><span class="track-title"> </span> <span class="track-sub"> </span></div> <span class="text-[12px] text-[var(--text-muted)] numeric mr-1"> </span> <button title="Favorite"><!></button> <button class="icon-btn" title="Add to queue"><!></button></div>'),Nv=L('<section class="mt-6 flex flex-col gap-3" id="trending-hits"><div class="flex items-center justify-between gap-3"><h2 class="section-title"> </h2> <div class="inline-flex gap-1 p-1 rounded-full bg-[var(--surface)] border border-[var(--border)]"><button> </button> <button> </button></div></div> <div class="track-list"></div></section>'),Mv=L('<div class="py-12 text-center"><p class="text-[var(--text-muted)]">Connect Tauri runtime (`cargo tauri dev`) to load live YouTube Music feeds.</p></div>');function Ev(t,e){Ae(e,!0);let r=G("trending"),a=J(()=>n(r)==="top"?c.homeFeed?.top??[]:c.homeFeed?.trending??[]);var i=se(),o=K(i);{var s=v=>{var f=Nv(),x=u(f),g=u(x),k=T(g,!0),_=d(g,2),S=u(_),I=T(S,!0),$=d(S,2),y=T($,!0),b=d(x,2);Re(b,23,()=>n(a),w=>w.id,(w,A,N)=>{const R=J(()=>c.currentTrack?.id===n(A).id);var p=zv();let j;var E=u(p),C=u(E);{var q=ae=>{vr(ae,{size:15,class:"text-[var(--accent-bright)] mx-auto"})},U=ae=>{var ge=Or();O(Te=>m(ge,Te),[()=>String(n(N)+1).padStart(2,"0")]),h(ae,ge)};Q(C,ae=>{n(R)?ae(q):ae(U,-1)})}var X=d(E,2);nt(X,{class:"track-cover",get src(){return n(A).cover_url},alt:""});var B=d(X,2),W=u(B),H=T(W,!0),F=d(W,2),D=T(F,!0),V=d(B,2),Z=T(V,!0),Y=d(V,2);let pe;var de=u(Y);{let ae=J(()=>c.isFavorite(n(A).id)?"currentColor":"none");gt(de,{size:16,get fill(){return n(ae)}})}var oe=d(Y,2),re=u(oe);yr(re,{size:16}),O((ae,ge)=>{j=ee(p,1,"track-row cursor-pointer",null,j,{active:n(R)}),m(H,n(A).title),m(D,n(A).artist),m(Z,ae),pe=ee(Y,1,"icon-btn",null,pe,{"fav-active":ge})},[()=>zt(n(A).duration_secs),()=>c.isFavorite(n(A).id)]),M("click",p,()=>c.replaceQueueAndPlay(n(a),n(N),"Trending hits")),M("keydown",p,ae=>ae.key==="Enter"&&c.replaceQueueAndPlay(n(a),n(N),"Trending hits")),M("click",Y,ae=>{ae.stopPropagation(),c.toggleFavorite(n(A))}),M("click",oe,ae=>{ae.stopPropagation(),c.enqueueTrack(n(A))}),h(w,p)}),O((w,A,N)=>{m(k,w),ee(S,1,`px-3 py-1 rounded-full text-[12px] font-semibold transition-colors ${n(r)==="trending"?"bg-[var(--accent-soft)] text-[var(--accent-bright)]":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),m(I,A),ee($,1,`px-3 py-1 rounded-full text-[12px] font-semibold transition-colors ${n(r)==="top"?"bg-[var(--accent-soft)] text-[var(--accent-bright)]":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),m(y,N)},[()=>P("home.trending_hits"),()=>P("home.tab_trending"),()=>P("home.tab_top")]),M("click",S,()=>z(r,"trending")),M("click",$,()=>z(r,"top")),h(v,f)},l=v=>{var f=Mv();h(v,f)};Q(o,v=>{n(a).length>0?v(s):c.homeLoading||v(l,1)})}h(t,i),ze()}Ve(["click","keydown"]);var Cv=L('<span class="absolute top-2 left-2 px-2 py-0.5 rounded-md text-[11px] font-bold bg-black/65 text-white numeric"> </span>'),Iv=L('<button class="p-2.5 rounded-2xl bg-[var(--surface)] border border-[var(--border)] hover:bg-[var(--surface-hover)] hover:border-[var(--border-strong)] text-left flex flex-col gap-1.5 transition-colors"><div class="relative w-full"><!> <!></div> <strong class="text-[13.5px] truncate w-full"> </strong> <span class="text-[12px] text-[var(--text-muted)] truncate w-full"> </span></button>'),Lv=L('<div class="mt-6"><!></div>'),Rv=L('<div class="page-clamp"><div class="flex items-center justify-between mb-4"><div><h1 class="text-[28px] font-extrabold tracking-[-0.025em] leading-tight"> </h1> <p class="text-[13px] text-[var(--text-muted)] mt-0.5"> </p></div> <button class="icon-btn"><!></button></div> <!> <!> <!> <!> <!> <!></div>');function jv(t,e){Ae(e,!0);let r=J(()=>{const j=new Date().getHours();return j>=5&&j<12?P("home.morning"):j>=12&&j<19?P("home.afternoon"):P("home.evening")}),a=J(()=>c.recentTracks.slice(0,6)),i=J(()=>c.homeFeed?.new_albums[0]??null),o=J(()=>c.homeFeed?c.homeFeed.new_albums.slice(c.homeFeed.new_albums.length>1?1:0,16):[]),s=J(()=>c.genres.slice(0,6));var l=Rv(),v=u(l),f=u(v),x=u(f),g=T(x,!0),k=d(x,2),_=T(k,!0),S=d(f,2),I=u(S);{let j=J(()=>c.homeLoading?"animate-spin":"");Hc(I,{size:16,get class(){return n(j)}})}var $=d(v,2);kv($,{});var y=d($,2);yv(y,{get tracks(){return n(a)}});var b=d(y,2);{var w=j=>{bv(j,{get album(){return n(i)}})};Q(b,j=>{n(i)&&j(w)})}var A=d(b,2);{var N=j=>{var E=Lv(),C=u(E);{let q=J(()=>P("home.new_releases"));Er(C,{get title(){return n(q)},id:"new-releases",children:(U,X)=>{var B=se(),W=K(B);Re(W,17,()=>n(o),H=>H.id,(H,F)=>{var D=Iv(),V=u(D),Z=u(V);nt(Z,{class:"media-cover",get src(){return n(F).cover_url},get alt(){return n(F).title}});var Y=d(Z,2);{var pe=ge=>{var Te=Cv(),ce=T(Te,!0);O(()=>m(ce,n(F).year)),h(ge,Te)};Q(Y,ge=>{n(F).year&&ge(pe)})}var de=d(V,2),oe=T(de,!0),re=d(de,2),ae=T(re,!0);O(()=>{m(oe,n(F).title),m(ae,n(F).artist)}),M("click",D,()=>c.openAlbum(n(F).id,n(F).title,n(F).cover_url)),h(H,D)}),h(U,B)},$$slots:{default:!0}})}h(j,E)};Q(A,j=>{n(o).length>0&&j(N)})}var R=d(A,2);Ev(R,{});var p=d(R,2);Av(p,{get stations(){return n(s)}}),O((j,E)=>{m(g,n(r)),m(_,j),le(S,"title",E)},[()=>P("home.subtitle"),()=>P("home.reload")]),M("click",S,()=>void c.loadHome(!0)),h(t,l),ze()}Ve(["click"]);var un=L('<span class="truncate"> </span>'),qv=L('<div class="px-2.5 pt-1 pb-1.5 text-[10.5px] font-bold uppercase tracking-[0.09em] text-[var(--text-muted)]"> </div>'),Ci=L('<span class="min-w-0 flex-1"><span class="block text-[13px] font-semibold truncate"> </span> <span class="block text-[11px] text-[var(--text-muted)] numeric"> </span></span>'),Ov=L('<button><span class="w-7 h-7 rounded-lg inline-flex items-center justify-center text-white shrink-0"><!></span> <!></button>'),Dv=L('<aside><nav class="p-3 space-y-1" aria-label="Main navigation"><button><!> <!></button> <button><!> <!></button> <button><!> <!></button></nav> <div class="mx-3 border-t border-[var(--border)]"></div> <div class="flex-1 min-h-0 overflow-y-auto scroll-y p-3 space-y-1"><!> <button><span class="w-7 h-7 rounded-lg inline-flex items-center justify-center text-white shrink-0" style="background: linear-gradient(135deg, #e01b24, #ff6b81);"><!></span> <!></button> <!></div></aside> <nav class="md:hidden fixed bottom-0 left-0 right-0 z-30 min-h-14 grid grid-cols-3 bg-[var(--bg-elevated)]/95 backdrop-blur-lg border-t border-[var(--border)] px-2" style="padding-bottom: env(safe-area-inset-bottom);" aria-label="Mobile navigation"><button><!> <span> </span></button> <button><!> <span> </span></button> <button><!> <span> </span></button></nav>',1);function Fv(t,e){Ae(e,!0);let r=J(()=>c.navCollapsed);var a=Dv(),i=K(a),o=u(i),s=u(o),l=u(s);gi(l,{size:19,class:"shrink-0"});var v=d(l,2);{var f=oe=>{var re=un(),ae=T(re,!0);O(ge=>m(ae,ge),[()=>P("nav.home")]),h(oe,re)};Q(v,oe=>{n(r)||oe(f)})}var x=d(s,2),g=u(x);xa(g,{size:19,class:"shrink-0"});var k=d(g,2);{var _=oe=>{var re=un(),ae=T(re,!0);O(ge=>m(ae,ge),[()=>P("nav.explore")]),h(oe,re)};Q(k,oe=>{n(r)||oe(_)})}var S=d(x,2),I=u(S);bi(I,{size:19,class:"shrink-0"});var $=d(I,2);{var y=oe=>{var re=un(),ae=T(re,!0);O(ge=>m(ae,ge),[()=>P("nav.collection")]),h(oe,re)};Q($,oe=>{n(r)||oe(y)})}var b=d(o,4),w=u(b);{var A=oe=>{var re=qv(),ae=T(re,!0);O(ge=>m(ae,ge),[()=>P("collection.local_playlists")]),h(oe,re)};Q(w,oe=>{n(r)||oe(A)})}var N=d(w,2),R=u(N),p=u(R);gt(p,{size:13,fill:"currentColor"});var j=d(R,2);{var E=oe=>{var re=Ci(),ae=u(re),ge=T(ae,!0),Te=d(ae,2),ce=T(Te,!0);O((be,me)=>{m(ge,be),m(ce,me)},[()=>P("collection.favorites"),()=>c.favorites.length===1?P("collection.song_one"):P("collection.song_many",{count:c.favorites.length})]),h(oe,re)};Q(j,oe=>{n(r)||oe(E)})}var C=d(N,2);Re(C,17,()=>c.userPlaylists,oe=>oe.id,(oe,re)=>{var ae=Ov(),ge=u(ae),Te=u(ge);Wr(Te,{get name(){return n(re).icon_name},size:13});var ce=d(ge,2);{var be=me=>{var Ee=Ci(),we=u(Ee),qe=T(we,!0),We=d(we,2),Ye=T(We,!0);O(vt=>{m(qe,n(re).name),m(Ye,vt)},[()=>n(re).tracks.length===1?P("collection.song_one"):P("collection.song_many",{count:n(re).tracks.length})]),h(me,Ee)};Q(ce,me=>{n(r)||me(be)})}O(()=>{ee(ae,1,`w-full flex items-center gap-2.5 rounded-xl px-2.5 py-2 text-left transition-colors hover:bg-[var(--surface)] ${n(r)?"justify-center px-0":""}`),le(ae,"title",n(re).name),Ke(ge,`background: ${(n(re).color_hex||"#2190a4")??""};`)}),M("click",ae,()=>{n(re).tracks.length>0?c.replaceQueueAndPlay(n(re).tracks,0,n(re).name):c.navigateTab("collection")}),h(oe,ae)});var q=d(i,2),U=u(q),X=u(U);gi(X,{size:19});var B=d(X,2),W=T(B,!0),H=d(U,2),F=u(H);xa(F,{size:19});var D=d(F,2),V=T(D,!0),Z=d(H,2),Y=u(Z);bi(Y,{size:19});var pe=d(Y,2),de=T(pe,!0);O((oe,re,ae,ge,Te,ce,be)=>{ee(i,1,`hidden md:flex flex-col shrink-0 h-full border-r border-[var(--border)] bg-[var(--bg-sidebar)] select-none transition-[width] duration-200 ease-out overflow-hidden ${n(r)?"w-[72px]":"w-[228px]"}`),ee(s,1,`w-full flex items-center gap-3 rounded-xl px-3 py-2.5 text-[13.5px] font-semibold transition-all active:scale-[0.98] ${n(r)?"justify-center px-0":""} ${c.activeTab==="home"?"bg-[var(--accent-soft)] text-[var(--accent-bright)] shadow-sm":"text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]"}`),le(s,"title",oe),ee(x,1,`w-full flex items-center gap-3 rounded-xl px-3 py-2.5 text-[13.5px] font-semibold transition-all active:scale-[0.98] ${n(r)?"justify-center px-0":""} ${c.activeTab==="explore"?"bg-[var(--accent-soft)] text-[var(--accent-bright)] shadow-sm":"text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]"}`),le(x,"title",re),ee(S,1,`w-full flex items-center gap-3 rounded-xl px-3 py-2.5 text-[13.5px] font-semibold transition-all active:scale-[0.98] ${n(r)?"justify-center px-0":""} ${c.activeTab==="collection"?"bg-[var(--accent-soft)] text-[var(--accent-bright)] shadow-sm":"text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]"}`),le(S,"title",ae),ee(N,1,`w-full flex items-center gap-2.5 rounded-xl px-2.5 py-2 text-left transition-colors hover:bg-[var(--surface)] ${n(r)?"justify-center px-0":""}`),le(N,"title",ge),ee(U,1,`flex flex-col items-center justify-center gap-0.5 text-[11px] font-semibold ${c.activeTab==="home"?"text-[var(--accent-bright)]":"text-[var(--text-muted)]"}`),m(W,Te),ee(H,1,`flex flex-col items-center justify-center gap-0.5 text-[11px] font-semibold ${c.activeTab==="explore"?"text-[var(--accent-bright)]":"text-[var(--text-muted)]"}`),m(V,ce),ee(Z,1,`flex flex-col items-center justify-center gap-0.5 text-[11px] font-semibold ${c.activeTab==="collection"?"text-[var(--accent-bright)]":"text-[var(--text-muted)]"}`),m(de,be)},[()=>P("nav.home"),()=>P("nav.explore"),()=>P("nav.collection"),()=>P("collection.favorites"),()=>P("nav.home"),()=>P("nav.explore"),()=>P("nav.collection")]),M("click",s,()=>c.navigateTab("home")),M("click",x,()=>c.navigateTab("explore")),M("click",S,()=>c.navigateTab("collection")),M("click",N,()=>{c.favorites.length>0?c.replaceQueueAndPlay(c.favorites,0,P("collection.favorites")):c.navigateTab("collection")}),M("click",U,()=>c.navigateTab("home")),M("click",H,()=>c.navigateTab("explore")),M("click",Z,()=>c.navigateTab("collection")),h(t,a),ze()}Ve(["click"]);var Hv=L("<div><!></div>"),Ii=L('<span class="w-1 h-1 rounded-full bg-[var(--accent-bright)] absolute bottom-1"></span>'),Vv=L('<button class="w-[340px] flex items-center gap-3 p-3 rounded-2xl bg-white/[0.05] hover:bg-white/[0.08] border border-white/10 text-left transition-colors"><!> <div class="min-w-0 flex-1"><span class="block text-[10px] font-extrabold uppercase tracking-[0.1em] text-[var(--text-muted)]"> </span> <span class="block text-[13.5px] font-bold truncate"> </span> <span class="block text-[12px] text-[var(--text-muted)] truncate"> </span></div></button>'),Qv=L('<span class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[11px] font-bold bg-[var(--accent-soft)] text-[var(--accent-bright)]"><span class="w-1.5 h-1.5 rounded-full bg-[var(--accent-bright)]"></span> </span>'),Uv=L('<span class="px-2.5 py-0.5 rounded-full text-[11px] font-semibold bg-white/8 text-[var(--text-muted)]"> </span>'),Bv=L("<button> </button>"),Wv=L('<button class="absolute bottom-6 left-1/2 -translate-x-1/2 inline-flex items-center gap-2 px-4 py-2 rounded-full text-[13px] font-bold bg-[var(--accent)] hover:bg-[var(--accent-bright)] text-white shadow-[0_10px_28px_rgba(0,0,0,0.5)] transition-all active:scale-95"><!> </button>'),Gv=L('<div class="relative flex-1 min-h-0 flex flex-col"><div role="region" aria-label="Lyrics" class="party-lyrics-scroll flex-1 min-h-0 overflow-y-auto py-[36vh] pr-8 pl-4 flex flex-col gap-8 svelte-1o5omgn"></div> <!></div>'),Yv=L('<div class="party-lyrics-scroll flex-1 min-h-0 overflow-y-auto py-12 pr-6 svelte-1o5omgn"><pre class="whitespace-pre-wrap font-sans text-[24px] font-bold leading-[1.6] tracking-[-0.02em] text-white/80"> </pre></div>'),Kv=L('<div class="flex-1 flex flex-col items-center justify-center text-center gap-3 text-[var(--text-muted)]"><!> <p class="text-[18px] font-semibold"> </p></div>'),Jv=L('<div class="fixed inset-0 z-50 flex flex-col bg-[var(--bg)] text-[var(--text)] select-none overflow-hidden" style="background-image: radial-gradient(ellipse at 26% 36%, rgba(var(--ambient-rgb), 0.34) 0%, rgba(var(--ambient-rgb), 0.1) 42%, transparent 74%);"><header class="shrink-0 flex items-center justify-between px-4 sm:px-7 py-4 sm:py-5"><div class="flex items-center gap-3 min-w-0"><span class="inline-flex items-center gap-1.5 px-3.5 py-1.5 rounded-full text-[12px] font-bold tracking-wide bg-[var(--accent-soft)] text-[var(--accent-bright)] border border-[var(--accent-border)]/50"><!> </span> <span class="text-[13.5px] font-medium text-[var(--text-muted)] truncate"> </span></div> <button class="inline-flex items-center gap-1.5 px-4 py-1.5 rounded-full text-[13px] font-semibold bg-white/8 hover:bg-white/14 border border-white/10 transition-colors"><!> </button></header> <div class="flex-1 min-h-0 overflow-y-auto lg:overflow-hidden grid grid-cols-1 lg:grid-cols-[440px_minmax(0,1fr)] gap-8 lg:gap-10 px-5 sm:px-8 lg:px-10 pb-8 max-w-[1440px] w-full mx-auto"><div class="flex flex-col items-center justify-center gap-5"><!> <div class="w-[340px] text-center space-y-1"><h1 class="text-[28px] font-black tracking-[-0.03em] leading-tight line-clamp-2"> </h1> <p class="text-[16px] font-medium text-[var(--text-muted)] truncate"> </p></div> <div class="w-[340px] space-y-2.5"><div class="h-1.5 w-full rounded-full bg-white/15 cursor-pointer overflow-hidden group" role="slider" tabindex="0" aria-label="Seek"><div class="h-full rounded-full bg-[var(--accent-bright)] transition-[width] duration-150 ease-linear"></div></div> <div class="flex items-center justify-between text-[12.5px] font-medium text-[var(--text-muted)] numeric"><span> </span> <button><!></button> <span> </span></div></div> <div class="flex items-center justify-center gap-4"><button title="Shuffle"><!> <!></button> <button class="icon-btn !w-11 !h-11" title="Previous"><!></button> <button class="play-circle !w-[62px] !h-[62px]" title="Play / Pause"><!></button> <button class="icon-btn !w-11 !h-11" title="Next"><!></button> <button title="Repeat"><!> <!></button></div> <!></div> <div class="min-h-0 flex flex-col"><div class="shrink-0 flex items-center gap-2.5 pb-2"><span class="text-[13px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)]"> </span> <!></div> <!></div></div></div>');function Xv(t,e){Ae(e,!0);let r=Je({}),a=G(!1),i=G(null),o=J(()=>c.currentTrack?c.isFavorite(c.currentTrack.id):!1),s=J(()=>c.durationSecs>0?Math.max(0,Math.min(100,c.positionSecs/c.durationSecs*100)):0);$t(()=>{const ie=c.currentTrack?.id??null;ie!==n(i)&&(z(i,ie,!0),z(a,!1))}),$t(()=>{const ie=c.activeLyricIndex;ie!=null&&c.partyMode&&!n(a)&&r[ie]?.scrollIntoView({behavior:"smooth",block:"center"})});function l(){z(a,!0)}function v(){z(a,!1);const ie=c.activeLyricIndex;ie!=null&&r[ie]?.scrollIntoView({behavior:"smooth",block:"center"})}function f(ie){if(c.activeLyricIndex==null)return .6;if(ie===c.activeLyricIndex)return 1;const _e=Math.abs(ie-c.activeLyricIndex);return _e===1?.36:_e===2?.2:.11}function x(ie){return c.activeLyricIndex==null?!1:Math.abs(ie-c.activeLyricIndex)>=2}function g(ie){if(!c.currentTrack||c.durationSecs<=0)return;const _e=ie.currentTarget.getBoundingClientRect(),De=Math.max(0,Math.min(1,(ie.clientX-_e.left)/_e.width));c.seek(De*c.durationSecs)}var k=Jv(),_=u(k),S=u(_),I=u(S),$=u(I);Bn($,{size:14});var y=d($),b=d(I,2),w=T(b,!0),A=d(S,2),N=u(A);Cr(N,{size:15});var R=d(N),p=d(_,2),j=u(p),E=u(j);Hn(E,()=>c.currentTrack?.id??"none",ie=>{var _e=Hv(),De=u(_e);{let te=J(()=>c.currentTrack?.cover_large_url),xe=J(()=>c.currentTrack?.cover_url),Ie=J(()=>c.currentTrack?.title??"");nt(De,{class:"party-cover !w-[210px] !h-[210px] sm:!w-[260px] sm:!h-[260px] lg:!w-[292px] lg:!h-[292px] !rounded-[22px] border border-white/12 shadow-[0_26px_64px_rgba(0,0,0,0.58)]",get src(){return n(te)},get fallbackSrc(){return n(xe)},get alt(){return n(Ie)},eager:!0})}ot(1,_e,()=>Ir,()=>({start:.94,duration:280,easing:At})),h(ie,_e)});var C=d(E,2),q=u(C),U=T(q,!0),X=d(q,2),B=T(X,!0),W=d(C,2),H=u(W);le(H,"aria-valuemin",0);var F=T(H),D=d(H,2),V=u(D),Z=T(V,!0),Y=d(V,2),pe=u(Y);{let ie=J(()=>n(o)?"currentColor":"none");gt(pe,{size:18,get fill(){return n(ie)}})}var de=d(Y,2),oe=T(de,!0),re=d(W,2),ae=u(re),ge=u(ae);ta(ge,{size:18});var Te=d(ge,2);{var ce=ie=>{var _e=Ii();h(ie,_e)};Q(Te,ie=>{c.shuffle&&ie(ce)})}var be=d(ae,2),me=u(be);Gn(me,{size:22,fill:"currentColor"});var Ee=d(be,2),we=u(Ee);{var qe=ie=>{Wn(ie,{size:24,fill:"currentColor"})},We=ie=>{bt(ie,{size:24,fill:"currentColor",class:"translate-x-[1.5px]"})};Q(we,ie=>{c.isPlaying?ie(qe):ie(We,-1)})}var Ye=d(Ee,2),vt=u(Ye);Yn(vt,{size:22,fill:"currentColor"});var lt=d(Ye,2),ye=u(lt);Js(ye,{size:18});var Pe=d(ye,2);{var ke=ie=>{var _e=Ii();h(ie,_e)};Q(Pe,ie=>{c.repeat&&ie(ke)})}var Me=d(re,2);{var Oe=ie=>{var _e=Vv(),De=u(_e);nt(De,{class:"up-next-cover !w-11 !h-11 !rounded-lg",get src(){return c.nextTrack.cover_url},alt:""});var te=d(De,2),xe=u(te),Ie=T(xe,!0),Qe=d(xe,2),Ze=T(Qe,!0),xt=d(Qe,2),mt=T(xt,!0);O(Mt=>{m(Ie,Mt),m(Ze,c.nextTrack.title),m(mt,c.nextTrack.artist)},[()=>P("party.up_next")]),M("click",_e,()=>void c.next()),h(ie,_e)};Q(Me,ie=>{c.nextTrack&&ie(Oe)})}var ne=d(j,2),Se=u(ne),Ce=u(Se),$e=T(Ce,!0),Be=d(Ce,2);{var tt=ie=>{var _e=Qv(),De=d(u(_e));O(te=>m(De,` ${te??""}`),[()=>P("lyrics.synced")]),h(ie,_e)},Xe=ie=>{var _e=Uv(),De=T(_e,!0);O(te=>m(De,te),[()=>P("lyrics.static")]),h(ie,_e)};Q(Be,ie=>{c.lyrics.type==="Synced"?ie(tt):c.lyrics.type==="Plain"&&ie(Xe,1)})}var st=d(Se,2);{var Fe=ie=>{var _e=Gv(),De=u(_e);Re(De,21,()=>c.lyrics.lines,Rt,(Ie,Qe,Ze)=>{const xt=J(()=>c.activeLyricIndex===Ze),mt=J(()=>Ze>0?c.lyrics.lines[Ze-1]:null),Mt=J(()=>n(mt)!=null&&n(Qe).time_secs-n(mt).time_secs>=5.5);var rt=Bv(),Pt=T(rt,!0);Xr(rt,(yt,ct)=>r[ct]=yt,yt=>r?.[yt],()=>[Ze]),O((yt,ct)=>{ee(rt,1,`party-line text-left origin-left ${n(xt)?"active-line":""} ${yt??""} ${n(Mt)?"mt-5":""}`,"svelte-1o5omgn"),Ke(rt,`opacity: ${ct??""};`),m(Pt,n(Qe).text)},[()=>x(Ze)?"far-line":"",()=>f(Ze)]),M("click",rt,()=>{z(a,!1),c.seek(n(Qe).time_secs)}),h(Ie,rt)});var te=d(De,2);{var xe=Ie=>{var Qe=Wv(),Ze=u(Qe);Ys(Ze,{size:15});var xt=d(Ze);O(mt=>m(xt,` ${mt??""}`),[()=>P("lyrics.sync_btn")]),M("click",Qe,v),ot(3,Qe,()=>Ir,()=>({start:.88,duration:180,easing:At})),h(Ie,Qe)};Q(te,Ie=>{n(a)&&c.activeLyricIndex!=null&&Ie(xe)})}_t("wheel",De,l),M("touchmove",De,l),h(ie,_e)},ht=ie=>{var _e=Yv(),De=u(_e),te=T(De,!0);O(()=>m(te,c.lyrics.text)),h(ie,_e)},dt=ie=>{var _e=Kv(),De=u(_e);ea(De,{size:40,class:"opacity-30"});var te=d(De,2),xe=T(te,!0);O(Ie=>m(xe,Ie),[()=>P("lyrics.none_track")]),h(ie,_e)};Q(st,ie=>{c.lyrics.type==="Synced"?ie(Fe):c.lyrics.type==="Plain"?ie(ht,1):ie(dt,-1)})}O((ie,_e,De,te,xe,Ie,Qe,Ze)=>{m(y,` ${ie??""}`),m(w,_e),m(R,` ${De??""}`),m(U,c.currentTrack?.title??"Nothing playing"),m(B,c.currentTrack?.artist??""),le(H,"aria-valuemax",te),le(H,"aria-valuenow",xe),Ke(F,`width: ${n(s)??""}%`),m(Z,Ie),ee(Y,1,`icon-btn !w-9 !h-9 transition-transform active:scale-90 ${n(o)?"text-[var(--coral)]":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),Y.disabled=!c.currentTrack,le(Y,"title",n(o)?"Remove from favorites":"Add to favorites"),m(oe,Qe),ee(ae,1,`icon-btn !w-10 !h-10 relative ${c.shuffle?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),be.disabled=!c.currentTrack,Ee.disabled=!c.currentTrack,Ye.disabled=!c.currentTrack,ee(lt,1,`icon-btn !w-10 !h-10 relative ${c.repeat?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),m($e,Ze)},[()=>P("party.badge"),()=>c.sourceName==="Not playing"?P("player.not_playing"):c.sourceName,()=>P("party.exit"),()=>Math.round(c.durationSecs),()=>Math.round(c.positionSecs),()=>zt(c.positionSecs),()=>zt(c.durationSecs),()=>P("lyrics.title")]),M("click",A,()=>c.partyMode=!1),M("click",H,g),M("keydown",H,ie=>{ie.key==="ArrowRight"?c.seek(c.positionSecs+5):ie.key==="ArrowLeft"&&c.seek(Math.max(0,c.positionSecs-5))}),M("click",Y,()=>c.currentTrack&&c.toggleFavorite(c.currentTrack)),M("click",ae,()=>c.toggleShuffle()),M("click",be,()=>void c.prev()),M("click",Ee,()=>void c.togglePlay()),M("click",Ye,()=>void c.next()),M("click",lt,()=>c.toggleRepeat()),h(t,k),ze()}Ve(["click","keydown","touchmove"]);var vn=L("<!> ",1),Zv=L('<button type="button"><!></button>'),ef=L('<button type="button"> </button>'),tf=L('<div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-md" role="presentation"><div role="dialog" aria-modal="true" class="w-[min(520px,calc(100vw-32px))] max-h-[calc(100vh-48px)] overflow-y-auto rounded-[18px] bg-[var(--bg-elevated)] text-[var(--text)] border border-[var(--border-strong)] shadow-[0_24px_64px_rgba(0,0,0,0.62)]"><div class="flex items-center justify-between px-6 py-4 border-b border-[var(--border)]"><h2 class="text-[18px] font-extrabold tracking-tight"> </h2> <button class="icon-btn" title="Close"><!></button></div> <div class="p-6 space-y-5"><section class="space-y-2.5"><h3 class="text-[11px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)]"> </h3> <div class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] divide-y divide-[var(--border)]"><div class="p-3.5 flex items-center justify-between gap-3"><div><div class="text-[14px] font-semibold"> </div> <div class="text-[12px] text-[var(--text-muted)]"> </div></div> <div class="inline-flex p-1 rounded-xl bg-black/20 border border-[var(--border)] gap-1 shrink-0"></div></div> <div class="p-3.5 flex items-center justify-between gap-3"><div><div class="flex items-center gap-1.5 text-[14px] font-semibold"><!> </div> <div class="text-[12px] text-[var(--text-muted)]"> </div></div> <div class="inline-flex p-1 rounded-xl bg-black/20 border border-[var(--border)] gap-1 shrink-0"></div></div></div></section> <section class="space-y-2.5"><h3 class="text-[11px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)]"> </h3> <div class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] divide-y divide-[var(--border)]"><div class="p-3.5 flex items-center justify-between gap-4"><div class="min-w-0"><div class="text-[14px] font-semibold"> </div> <div class="text-[12px] text-[var(--text-muted)] numeric"> </div></div> <input class="w-36 !h-1.5" type="range" min="0" max="10" step="1"/></div> <div class="p-3.5 flex items-center justify-between gap-4"><div class="min-w-0"><div class="text-[14px] font-semibold"> </div> <div class="text-[12px] text-[var(--text-muted)]"> </div></div> <button type="button" role="switch"><span></span></button></div> <div class="p-3.5 flex items-center justify-between gap-4"><div class="min-w-0"><div class="text-[14px] font-semibold"> </div> <div class="text-[12px] text-[var(--text-muted)]"> </div></div> <button type="button" role="switch"><span></span></button></div></div></section> <section class="space-y-2.5"><h3 class="text-[11px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)]"> </h3> <div class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] divide-y divide-[var(--border)]"><div class="p-3.5 flex items-center justify-between gap-4"><div class="min-w-0 flex-1 space-y-1.5"><div class="flex items-center gap-2 text-[14px] font-semibold"><!> </div> <div class="text-[12px] text-[var(--text-muted)] numeric"> </div> <div class="h-1.5 w-full max-w-[240px] rounded-full bg-white/10 overflow-hidden"><div class="h-full rounded-full bg-[var(--accent-bright)] transition-[width] duration-300"></div></div></div> <button class="pill-btn hover:!bg-red-500/20 hover:!border-red-500/40 hover:!text-red-400 shrink-0"><!> </button></div> <div class="p-3.5 flex items-center justify-between gap-4"><div class="min-w-0"><div class="text-[14px] font-semibold"> </div> <div class="text-[12px] text-[var(--text-muted)] numeric"> </div></div> <button class="pill-btn shrink-0"> </button></div></div></section></div></div></div>');function rf(t,e){Ae(e,!0);let r=G(0);$t(()=>{c.settingsOpen&&Ue()&&Ne.getCacheBytes().then(_=>z(r,_,!0)).catch(()=>{})});let a=J(()=>(n(r)/(1024*1024)).toFixed(1)),i=J(()=>Math.min(100,n(r)/(1024*1024*1024)*100)),o=J(()=>Math.round(c.crossfadeSecs/10*100));async function s(){Ue()&&(await Ne.clearAudioCache(),z(r,0),c.showToast("Audio cache cleared"))}function l(_){c.settingsOpen&&_.key==="Escape"&&(_.preventDefault(),c.settingsOpen=!1)}const v=["system","dark","light"],f=[{id:"Auto",label:"Auto"},{id:"Es",label:"Español"},{id:"En",label:"English"}];var x=se();_t("keydown",ga,l);var g=K(x);{var k=_=>{var S=tf(),I=u(S),$=u(I),y=u($),b=T(y,!0),w=d(y,2),A=u(w);Cr(A,{size:17});var N=d($,2),R=u(N),p=u(R),j=T(p,!0),E=d(p,2),C=u(E),q=u(C),U=u(q),X=T(U,!0),B=d(U,2),W=T(B,!0),H=d(q,2);Re(H,21,()=>v,Rt,(Jt,Ht)=>{var Vt=Zv(),aa=u(Vt);{var na=Qt=>{var Xt=vn(),Zt=K(Xt);Oc(Zt,{size:13});var Sr=d(Zt);O($r=>m(Sr,` ${$r??""}`),[()=>P("settings.theme_system")]),h(Qt,Xt)},Ja=Qt=>{var Xt=vn(),Zt=K(Xt);Un(Zt,{size:13});var Sr=d(Zt);O($r=>m(Sr,` ${$r??""}`),[()=>P("settings.theme_dark")]),h(Qt,Xt)},Xa=Qt=>{var Xt=vn(),Zt=K(Xt);Xs(Zt,{size:13});var Sr=d(Zt);O($r=>m(Sr,` ${$r??""}`),[()=>P("settings.theme_light")]),h(Qt,Xt)};Q(aa,Qt=>{n(Ht)==="system"?Qt(na):n(Ht)==="dark"?Qt(Ja,1):Qt(Xa,-1)})}O(()=>ee(Vt,1,`inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-[12px] font-semibold transition-all ${c.themeMode===n(Ht)?"bg-[var(--accent)] text-white shadow-sm":"text-[var(--text-muted)] hover:text-[var(--text)]"}`)),M("click",Vt,()=>c.setThemeMode(n(Ht))),h(Jt,Vt)});var F=d(C,2),D=u(F),V=u(D),Z=u(V);Bs(Z,{size:14,class:"text-[var(--accent-bright)]"});var Y=d(Z),pe=d(V,2),de=T(pe,!0),oe=d(D,2);Re(oe,21,()=>f,Rt,(Jt,Ht)=>{var Vt=ef(),aa=T(Vt,!0);O(na=>{ee(Vt,1,`px-2.5 py-1.5 rounded-lg text-[12px] font-semibold transition-all ${c.language===n(Ht).id?"bg-[var(--accent)] text-white shadow-sm":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),m(aa,na)},[()=>n(Ht).id==="Auto"?P("settings.lang_auto"):n(Ht).label]),M("click",Vt,()=>c.setLanguage(n(Ht).id)),h(Jt,Vt)});var re=d(R,2),ae=u(re),ge=T(ae,!0),Te=d(ae,2),ce=u(Te),be=u(ce),me=u(be),Ee=T(me,!0),we=d(me,2),qe=T(we,!0),We=d(be,2),Ye=d(ce,2),vt=u(Ye),lt=u(vt),ye=T(lt,!0),Pe=d(lt,2),ke=T(Pe,!0),Me=d(vt,2),Oe=T(Me),ne=d(Ye,2),Se=u(ne),Ce=u(Se),$e=T(Ce,!0),Be=d(Ce,2),tt=T(Be,!0),Xe=d(Se,2),st=T(Xe),Fe=d(re,2),ht=u(Fe),dt=T(ht,!0),ie=d(ht,2),_e=u(ie),De=u(_e),te=u(De),xe=u(te);Lc(xe,{size:15,class:"text-[var(--accent-bright)]"});var Ie=d(xe),Qe=d(te,2),Ze=T(Qe,!0),xt=d(Qe,2),mt=T(xt),Mt=d(De,2),rt=u(Mt);Kn(rt,{size:13});var Pt=d(rt),yt=d(_e,2),ct=u(yt),fr=u(ct),ra=T(fr,!0),Tt=d(fr,2),ao=T(Tt,!0),Ka=d(ct,2),no=T(Ka,!0);O((Jt,Ht,Vt,aa,na,Ja,Xa,Qt,Xt,Zt,Sr,$r,io,so,oo,lo,co,uo,vo,fo,ho,po,_o,go)=>{le(I,"aria-label",Jt),m(b,Ht),m(j,Vt),m(X,aa),m(W,na),m(Y,` ${Ja??""}`),m(de,Xa),m(ge,Qt),m(Ee,Xt),m(qe,Zt),Tn(We,c.crossfadeSecs),le(We,"aria-label",Sr),Ke(We,`background: linear-gradient(to right, var(--accent-bright) ${n(o)??""}%, rgba(255,255,255,0.16) ${n(o)??""}%);`),m(ye,$r),m(ke,io),le(Me,"aria-checked",c.autoplayRelated),le(Me,"aria-label",so),ee(Me,1,`w-11 h-6 rounded-full p-0.5 transition-colors shrink-0 ${c.autoplayRelated?"bg-[var(--accent)]":"bg-white/15"}`),ee(Oe,1,`block w-5 h-5 rounded-full bg-white shadow transition-transform ${c.autoplayRelated?"translate-x-5":"translate-x-0"}`),m($e,oo),m(tt,lo),le(Xe,"aria-checked",c.backgroundPlayback),le(Xe,"aria-label",co),ee(Xe,1,`w-11 h-6 rounded-full p-0.5 transition-colors shrink-0 ${c.backgroundPlayback?"bg-[var(--accent)]":"bg-white/15"}`),ee(st,1,`block w-5 h-5 rounded-full bg-white shadow transition-transform ${c.backgroundPlayback?"translate-x-5":"translate-x-0"}`),m(dt,uo),m(Ie,` ${vo??""}`),m(Ze,fo),Ke(mt,`width: ${n(i)??""}%`),Mt.disabled=n(r)===0,m(Pt,` ${ho??""}`),m(ra,po),m(ao,_o),Ka.disabled=c.searchHistory.length===0,m(no,go)},[()=>P("settings.title"),()=>P("settings.title"),()=>P("settings.appearance"),()=>P("settings.theme"),()=>P("settings.theme_sub"),()=>P("settings.language"),()=>P("settings.language_sub"),()=>P("settings.playback"),()=>P("settings.crossfade"),()=>c.crossfadeSecs===0?P("settings.crossfade_off"):P("settings.crossfade_secs",{secs:c.crossfadeSecs}),()=>P("settings.crossfade"),()=>P("settings.autoplay"),()=>P("settings.autoplay_sub"),()=>P("settings.autoplay"),()=>P("settings.background"),()=>P("settings.background_sub"),()=>P("settings.background"),()=>P("settings.storage"),()=>P("settings.cache"),()=>P("settings.cache_sub",{mb:n(a)}),()=>P("settings.clear"),()=>P("settings.search_history"),()=>c.searchHistory.length===1?P("settings.search_history_one"):P("settings.search_history_many",{count:c.searchHistory.length}),()=>P("settings.clear")]),M("click",S,Jt=>{Jt.target===Jt.currentTarget&&(c.settingsOpen=!1)}),M("click",w,()=>c.settingsOpen=!1),M("input",We,Jt=>c.setCrossfadeSecs(Number(Jt.currentTarget.value))),M("click",Me,()=>c.setAutoplay(!c.autoplayRelated)),M("click",Xe,()=>c.setBackgroundPlayback(!c.backgroundPlayback)),M("click",Mt,()=>void s()),M("click",Ka,()=>c.clearSearchHistory()),ot(3,I,()=>Ir,()=>({start:.95,duration:190,easing:At})),ot(3,S,()=>wr,()=>({duration:160})),h(_,S)};Q(g,_=>{c.settingsOpen&&_(k)})}h(t,x),ze()}Ve(["click","input"]);var af=L('<p class="text-[12px] text-[var(--text-muted)] truncate"> </p>'),nf=L('<span class="shrink-0 inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full text-[10.5px] font-bold bg-[var(--accent-soft)] text-[var(--accent-bright)]"><span class="w-1.5 h-1.5 rounded-full bg-[var(--accent-bright)]"></span> </span>'),sf=L('<span class="shrink-0 px-2.5 py-0.5 rounded-full text-[10.5px] font-semibold bg-[var(--surface)] text-[var(--text-muted)]"> </span>'),of=L("<button> </button>"),lf=L('<button class="absolute bottom-3 left-1/2 -translate-x-1/2 inline-flex items-center gap-1.5 px-3.5 py-1.5 rounded-full text-[12px] font-bold bg-[var(--accent)] hover:bg-[var(--accent-bright)] text-white shadow-[0_8px_24px_rgba(0,0,0,0.45)] transition-all active:scale-95"><!> </button>'),cf=L('<div class="relative flex-1 min-h-0 flex flex-col"><div role="region" aria-label="Lyrics" class="lyrics-fade flex-1 min-h-0 overflow-y-auto overflow-x-hidden px-2 pt-28 pb-36 flex flex-col gap-4 svelte-cw8tlx"></div> <!></div>'),df=L('<div class="lyrics-fade flex-1 min-h-0 overflow-y-auto py-8 svelte-cw8tlx"><pre class="whitespace-pre-wrap font-sans text-[14.5px] leading-relaxed text-[var(--text)]/85"> </pre></div>'),uf=L('<div class="flex-1 flex flex-col items-center justify-center text-center gap-2 py-12 text-[var(--text-muted)]"><!> <p class="text-[13px]"> </p></div>'),vf=L('<div class="flex flex-col gap-3 pt-1 flex-1 min-h-0"><div class="flex items-start justify-between gap-2 pb-2 border-b border-[var(--border)]"><div class="min-w-0"><h3 class="text-[14.5px] font-bold truncate"> </h3> <!></div> <!></div> <!></div>');function ff(t,e){Ae(e,!0);let r=Je({}),a=G(!1),i=G(null);$t(()=>{const p=c.currentTrack?.id??null;p!==n(i)&&(z(i,p,!0),z(a,!1))}),$t(()=>{const p=c.activeLyricIndex;p!=null&&!n(a)&&r[p]?.scrollIntoView({behavior:"smooth",block:"center"})});function o(){z(a,!0)}function s(){z(a,!1);const p=c.activeLyricIndex;p!=null&&r[p]?.scrollIntoView({behavior:"smooth",block:"center"})}function l(p){if(c.activeLyricIndex==null)return .65;if(p===c.activeLyricIndex)return 1;const j=Math.abs(p-c.activeLyricIndex);return j===1?.42:j===2?.24:.14}function v(p){return c.activeLyricIndex==null?!1:Math.abs(p-c.activeLyricIndex)>=2}var f=vf(),x=u(f),g=u(x),k=u(g),_=T(k,!0),S=d(k,2);{var I=p=>{var j=af(),E=T(j,!0);O(()=>m(E,c.currentTrack.artist)),h(p,j)};Q(S,p=>{c.currentTrack&&p(I)})}var $=d(g,2);{var y=p=>{var j=nf(),E=d(u(j));O(C=>m(E,` ${C??""}`),[()=>P("lyrics.synced")]),h(p,j)},b=p=>{var j=sf(),E=T(j,!0);O(C=>m(E,C),[()=>P("lyrics.static")]),h(p,j)};Q($,p=>{c.lyrics.type==="Synced"?p(y):c.lyrics.type==="Plain"&&p(b,1)})}var w=d(x,2);{var A=p=>{var j=cf(),E=u(j);Re(E,21,()=>c.lyrics.lines,Rt,(U,X,B)=>{const W=J(()=>c.activeLyricIndex===B),H=J(()=>B>0?c.lyrics.lines[B-1]:null),F=J(()=>n(H)!=null&&n(X).time_secs-n(H).time_secs>=5.5);var D=of(),V=T(D,!0);Xr(D,(Z,Y)=>r[Y]=Z,Z=>r?.[Z],()=>[B]),O((Z,Y)=>{ee(D,1,`sb-lyric-line text-left origin-left ${n(W)?"active-line":""} ${Z??""} ${n(F)?"mt-3":""}`,"svelte-cw8tlx"),Ke(D,`opacity: ${Y??""};`),m(V,n(X).text)},[()=>v(B)?"far-line":"",()=>l(B)]),M("click",D,()=>{z(a,!1),c.seek(n(X).time_secs)}),h(U,D)});var C=d(E,2);{var q=U=>{var X=lf(),B=u(X);Ys(B,{size:14});var W=d(B);O(H=>m(W,` ${H??""}`),[()=>P("lyrics.sync_btn")]),M("click",X,s),ot(3,X,()=>Ir,()=>({start:.88,duration:170,easing:At})),h(U,X)};Q(C,U=>{n(a)&&c.activeLyricIndex!=null&&U(q)})}_t("wheel",E,o),M("touchmove",E,o),h(p,j)},N=p=>{var j=df(),E=u(j),C=T(E,!0);O(()=>m(C,c.lyrics.text)),h(p,j)},R=p=>{var j=uf(),E=u(j);ea(E,{size:28,class:"opacity-40"});var C=d(E,2),q=T(C,!0);O(U=>m(q,U),[()=>c.currentTrack?P("lyrics.none_track"):P("lyrics.none_idle")]),h(p,j)};Q(w,p=>{c.lyrics.type==="Synced"?p(A):c.lyrics.type==="Plain"?p(N,1):p(R,-1)})}O(p=>m(_,p),[()=>c.currentTrack?.title??P("lyrics.title")]),ot(1,f,()=>wr,()=>({duration:180})),h(t,f),ze()}Ve(["touchmove","click"]);var hf=L("<div><!></div>"),pf=L('<p class="text-[12px] text-[var(--text-muted)]/75 truncate"> </p>'),_f=L('<span class="w-4 h-4 rounded-full border-2 border-current/30 border-t-current animate-spin"></span>'),gf=L('<span class="w-1.5 h-1.5 rounded-full bg-[var(--accent-bright)] absolute top-1 right-1"></span>'),bf=L('<button type="button" aria-label="Close popover" class="fixed inset-0 z-20 cursor-default bg-transparent"></button>'),xf=L('<div class="absolute bottom-12 left-1/2 -translate-x-1/2 w-[272px] p-3.5 rounded-2xl bg-[var(--bg-elevated)]/95 backdrop-blur-md border border-[var(--border-strong)] shadow-[0_16px_40px_rgba(0,0,0,0.5)] z-30 space-y-2.5"><div class="flex items-center justify-between text-[12px]"><span class="font-semibold text-[var(--text-muted)]"> </span> <span class="font-bold text-[var(--accent-bright)] numeric"> </span></div> <div class="flex items-center gap-3"><button><!></button> <input class="sidebar-seek flex-1 min-w-0 svelte-12h25u0" type="range" min="0" max="1" step="0.01" aria-label="Volume"/></div></div>'),mf=L('<span class="font-bold text-[var(--accent-bright)] numeric"> </span>'),yf=L('<button class="py-1.5 rounded-xl text-[12px] font-semibold bg-[var(--surface)] text-[var(--text-muted)] hover:bg-[var(--accent-soft)] hover:text-[var(--accent-bright)] transition-colors"> </button>'),wf=L('<div class="absolute bottom-12 left-1/2 -translate-x-1/2 w-[272px] p-3.5 rounded-2xl bg-[var(--bg-elevated)]/95 backdrop-blur-md border border-[var(--border-strong)] shadow-[0_16px_40px_rgba(0,0,0,0.5)] z-30 space-y-2.5"><div class="flex items-center justify-between text-[12px]"><span class="font-semibold text-[var(--text-muted)]"> </span> <!></div> <div class="grid grid-cols-5 gap-1.5"></div></div>'),Li=L('<span class="w-1 h-1 rounded-full bg-[var(--accent-bright)] absolute bottom-0.5"></span>'),kf=L('<span class="w-5 h-5 rounded-full border-2 border-white/30 border-t-white animate-spin"></span>'),Sf=L('<div class="flex-1 flex flex-col items-center justify-between py-4 gap-5"><div class="flex-1 flex items-center justify-center w-full py-2"><!></div> <div class="w-full flex flex-col items-center gap-4"><div class="w-full text-center px-1 space-y-1"><h2 class="text-[19px] font-extrabold leading-tight tracking-tight line-clamp-2"> </h2> <p class="text-[14px] font-medium text-[var(--text-muted)] truncate"> </p> <!></div> <div class="relative w-full flex items-center justify-center gap-5 py-0.5"><button><!></button> <button><!></button> <button title="Start radio from this song"><!></button> <button><!> <!></button> <button title="Party Mode"><!></button> <!> <!></div> <div class="w-full space-y-2.5"><input class="sidebar-seek w-full svelte-12h25u0" type="range" min="0" step="0.5" aria-label="Seek"/> <div class="flex items-center justify-between text-[12px] text-[var(--text-muted)] numeric"><span> </span> <span class="px-3 py-0.5 rounded-full text-[11px] font-bold bg-[var(--accent-soft)] text-[var(--accent-bright)]"> </span> <span> </span></div></div> <div class="w-full flex items-center justify-center gap-4 pt-1 pb-1"><button title="Shuffle"><!> <!></button> <button class="icon-btn !w-9 !h-9" title="Previous"><!></button> <button class="play-circle !w-[56px] !h-[56px] shrink-0" title="Play / Pause"><!></button> <button class="icon-btn !w-9 !h-9" title="Next"><!></button> <button title="Repeat"><!> <!></button></div></div></div>');function $f(t,e){Ae(e,!0);let r=G(!1),a=G(0),i=G(null),o=G(1),s=G(!1),l=J(()=>n(r)?n(a):c.positionSecs),v=J(()=>c.durationSecs>0?Math.max(0,Math.min(100,n(l)/c.durationSecs*100)):0),f=J(()=>Math.round(c.volume*100)),x=J(()=>c.currentTrack?c.isFavorite(c.currentTrack.id):!1);function g(te){z(i,n(i)===te?null:te,!0)}function k(){c.volume>.01?(z(o,c.volume,!0),c.setVolume(0)):c.setVolume(n(o)>.05?n(o):.8)}var _=Sf(),S=u(_),I=u(S);Hn(I,()=>c.currentTrack?.id??"none",te=>{var xe=hf(),Ie=u(xe);{let Qe=J(()=>c.currentTrack?.cover_large_url),Ze=J(()=>c.currentTrack?.cover_url),xt=J(()=>c.currentTrack?.title??"");nt(Ie,{class:"cover-large !w-[264px] !h-[264px] !rounded-2xl border border-white/10 shadow-[0_22px_50px_rgba(0,0,0,0.48)] transition-transform duration-300 hover:scale-[1.015]",get src(){return n(Qe)},get fallbackSrc(){return n(Ze)},get alt(){return n(xt)},eager:!0})}ot(1,xe,()=>Ir,()=>({start:.94,duration:260,easing:At})),h(te,xe)});var $=d(S,2),y=u($),b=u(y),w=T(b,!0),A=d(b,2),N=T(A,!0),R=d(A,2);{var p=te=>{var xe=pf(),Ie=T(xe,!0);O(Qe=>m(Ie,Qe),[()=>P("player.album_prefix",{album:c.currentTrack.album})]),h(te,xe)};Q(R,te=>{c.currentTrack?.album&&te(p)})}var j=d(y,2),E=u(j),C=u(E);{var q=te=>{yi(te,{size:18})},U=te=>{mi(te,{size:18})},X=te=>{vr(te,{size:18})};Q(C,te=>{c.volume===0?te(q):c.volume<.5?te(U,1):te(X,-1)})}var B=d(E,2),W=u(B);{let te=J(()=>n(x)?"currentColor":"none");gt(W,{size:18,get fill(){return n(te)}})}var H=d(B,2),F=u(H);{var D=te=>{var xe=_f();h(te,xe)},V=te=>{$a(te,{size:18})};Q(F,te=>{n(s)?te(D):te(V,-1)})}var Z=d(H,2),Y=u(Z);Wc(Y,{size:18});var pe=d(Y,2);{var de=te=>{var xe=gf();h(te,xe)};Q(pe,te=>{e.sleepRemainingSecs!=null&&te(de)})}var oe=d(Z,2),re=u(oe);Bn(re,{size:18});var ae=d(oe,2);{var ge=te=>{var xe=bf();M("click",xe,()=>z(i,null)),h(te,xe)};Q(ae,te=>{n(i)!==null&&te(ge)})}var Te=d(ae,2);{var ce=te=>{var xe=xf(),Ie=u(xe),Qe=u(Ie),Ze=T(Qe,!0),xt=d(Qe,2),mt=T(xt),Mt=d(Ie,2),rt=u(Mt),Pt=u(rt);{var yt=Tt=>{yi(Tt,{size:16})},ct=Tt=>{mi(Tt,{size:16})},fr=Tt=>{vr(Tt,{size:16})};Q(Pt,Tt=>{c.volume===0?Tt(yt):c.volume<.5?Tt(ct,1):Tt(fr,-1)})}var ra=d(rt,2);O(Tt=>{m(Ze,Tt),m(mt,`${n(f)??""}%`),ee(rt,1,`icon-btn !w-8 !h-8 shrink-0 bg-[var(--surface)] hover:bg-[var(--surface-hover)] ${c.volume===0?"text-[var(--coral)]":"text-[var(--text)]"}`),le(rt,"title",c.volume===0?"Unmute":"Mute"),Tn(ra,c.volume),Ke(ra,`background: linear-gradient(to right, var(--accent-bright) ${n(f)??""}%, rgba(255,255,255,0.16) ${n(f)??""}%);`)},[()=>P("player.volume")]),M("click",rt,k),M("input",ra,Tt=>c.setVolume(Number(Tt.currentTarget.value))),ot(3,xe,()=>pr,()=>({y:8,duration:170,easing:At})),h(te,xe)},be=te=>{var xe=wf(),Ie=u(xe),Qe=u(Ie),Ze=T(Qe,!0),xt=d(Qe,2);{var mt=rt=>{var Pt=mf(),yt=T(Pt,!0);O(ct=>m(yt,ct),[()=>zt(e.sleepRemainingSecs)]),h(rt,Pt)};Q(xt,rt=>{e.sleepRemainingSecs!=null&&rt(mt)})}var Mt=d(Ie,2);Re(Mt,20,()=>[null,15,30,45,60],Rt,(rt,Pt)=>{var yt=yf(),ct=T(yt,!0);O(fr=>m(ct,fr),[()=>Pt==null?P("player.sleep_off"):`${Pt}m`]),M("click",yt,()=>{e.onSetSleepTimer(Pt),z(i,null)}),h(rt,yt)}),O(rt=>m(Ze,rt),[()=>P("player.sleep_timer")]),ot(3,xe,()=>pr,()=>({y:8,duration:170,easing:At})),h(te,xe)};Q(Te,te=>{n(i)==="volume"?te(ce):n(i)==="timer"&&te(be,1)})}var me=d(j,2),Ee=u(me),we=d(Ee,2),qe=u(we),We=T(qe,!0),Ye=d(qe,2),vt=T(Ye,!0),lt=d(Ye,2),ye=T(lt,!0),Pe=d(me,2),ke=u(Pe),Me=u(ke);ta(Me,{size:17});var Oe=d(Me,2);{var ne=te=>{var xe=Li();h(te,xe)};Q(Oe,te=>{c.shuffle&&te(ne)})}var Se=d(ke,2),Ce=u(Se);Gn(Ce,{size:20,fill:"currentColor"});var $e=d(Se,2),Be=u($e);{var tt=te=>{var xe=kf();h(te,xe)},Xe=te=>{Wn(te,{size:22,fill:"currentColor"})},st=te=>{bt(te,{size:22,fill:"currentColor",class:"translate-x-[1.5px]"})};Q(Be,te=>{c.isLoadingAudio?te(tt):c.isPlaying?te(Xe,1):te(st,-1)})}var Fe=d($e,2),ht=u(Fe);Yn(ht,{size:20,fill:"currentColor"});var dt=d(Fe,2),ie=u(dt);Js(ie,{size:17});var _e=d(ie,2);{var De=te=>{var xe=Li();h(te,xe)};Q(_e,te=>{c.repeat&&te(De)})}O((te,xe,Ie,Qe,Ze,xt)=>{m(w,te),m(N,xe),ee(E,1,`icon-btn !w-9 !h-9 ${n(i)==="volume"||c.volume===0?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),le(E,"title",`Volume (${n(f)??""}%)`),ee(B,1,`icon-btn !w-9 !h-9 transition-transform active:scale-90 ${n(x)?"text-[var(--coral)]":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),B.disabled=!c.currentTrack,le(B,"title",n(x)?"Remove from favorites":"Add to favorites"),ee(H,1,`icon-btn !w-9 !h-9 ${n(s)?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),H.disabled=!c.currentTrack||n(s),ee(Z,1,`icon-btn !w-9 !h-9 relative ${n(i)==="timer"||e.sleepRemainingSecs!=null?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),le(Z,"title",Ie),ee(oe,1,`icon-btn !w-9 !h-9 ${c.partyMode?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),le(Ee,"max",Qe),Tn(Ee,n(l)),Ee.disabled=!c.currentTrack,Ke(Ee,`background: linear-gradient(to right, var(--accent-bright) ${n(v)??""}%, rgba(255,255,255,0.16) ${n(v)??""}%);`),m(We,Ze),m(vt,c.qualityLabel),m(ye,xt),ee(ke,1,`icon-btn relative ${c.shuffle?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),Se.disabled=!c.currentTrack,$e.disabled=!c.currentTrack,Fe.disabled=!c.currentTrack,ee(dt,1,`icon-btn relative ${c.repeat?"active":"text-[var(--text-muted)] hover:text-[var(--text)]"}`)},[()=>c.currentTrack?.title??P("player.nothing_playing"),()=>c.currentTrack?.artist??P("player.select_song"),()=>e.sleepRemainingSecs!=null?`Sleep timer: ${zt(e.sleepRemainingSecs)}`:"Sleep timer",()=>Math.max(1,c.durationSecs),()=>zt(n(l)),()=>zt(c.durationSecs)]),M("click",E,()=>g("volume")),M("click",B,()=>c.currentTrack&&c.toggleFavorite(c.currentTrack)),M("click",H,async()=>{z(i,null),z(s,!0);try{await c.startRadioFromCurrent()}finally{z(s,!1)}}),M("click",Z,()=>g("timer")),M("click",oe,()=>{z(i,null),c.partyMode=!0}),M("pointerdown",Ee,()=>{z(r,!0),z(a,c.positionSecs,!0)}),M("input",Ee,te=>{z(a,Number(te.currentTarget.value),!0)}),M("change",Ee,te=>{const xe=Number(te.currentTarget.value);z(r,!1),c.seek(xe)}),M("click",ke,()=>c.toggleShuffle()),M("click",Se,()=>void c.prev()),M("click",$e,()=>void c.togglePlay()),M("click",Fe,()=>void c.next()),M("click",dt,()=>c.toggleRepeat()),ot(1,_,()=>wr,()=>({duration:180})),h(t,_),ze()}Ve(["click","input","pointerdown","change"]);var Pf=L('<button class="px-2.5 py-1 rounded-full text-[11.5px] font-semibold bg-[var(--surface)] hover:bg-red-500/20 hover:text-red-400 transition-colors svelte-1567ky0"> </button>'),Tf=L('<div class="flex flex-col items-center justify-center text-center gap-2 py-10 text-[var(--text-muted)] svelte-1567ky0"><!> <p class="text-[13px] svelte-1567ky0"> </p></div>'),Af=L('<span class="eq-bars shrink-0 svelte-1567ky0" aria-label="Playing"><span class="svelte-1567ky0"></span><span class="svelte-1567ky0"></span><span class="svelte-1567ky0"></span></span>'),zf=L('<span class="text-[11px] text-[var(--text-muted)] numeric shrink-0 svelte-1567ky0"> </span>'),Nf=L('<div draggable="true" role="button" tabindex="0"><!> <!> <div class="track-meta svelte-1567ky0"><span> </span> <span class="track-sub !text-[11px] svelte-1567ky0"> </span></div> <!> <button class="icon-btn !w-6 !h-6 opacity-0 group-hover:opacity-100 transition-opacity shrink-0 svelte-1567ky0" title="Remove from queue"><!></button></div>'),Mf=L('<div class="track-list svelte-1567ky0"></div>'),Ef=L('<div class="flex items-center gap-2.5 px-2.5 py-2 border-b border-[var(--border)] last:border-b-0 hover:bg-[var(--surface-hover)] transition-colors svelte-1567ky0"><!> <div class="track-meta svelte-1567ky0"><span class="track-title !text-[12.5px] svelte-1567ky0"> </span> <span class="track-sub !text-[11px] svelte-1567ky0"> </span></div> <button class="icon-btn !w-7 !h-7 shrink-0 svelte-1567ky0" title="Add to queue"><!></button></div>'),Cf=L('<h4 class="text-[11.5px] font-bold uppercase tracking-wider text-[var(--text-muted)] mt-1 svelte-1567ky0"> </h4> <div class="track-list svelte-1567ky0"></div>',1),If=L('<div class="flex flex-col gap-3 pt-1 svelte-1567ky0"><div class="flex items-center justify-between gap-2 svelte-1567ky0"><h3 class="text-[14px] font-bold svelte-1567ky0"> <span class="text-[var(--text-muted)] font-medium svelte-1567ky0"> </span></h3> <!></div> <!> <div class="flex items-center justify-between gap-3 p-3 rounded-xl bg-[var(--surface)] border border-[var(--border)] svelte-1567ky0"><div class="min-w-0 svelte-1567ky0"><div class="text-[13px] font-bold svelte-1567ky0"> </div> <p class="text-[11.5px] text-[var(--text-muted)] leading-snug svelte-1567ky0"> </p></div> <button type="button" role="switch" aria-label="Toggle autoplay"><span></span></button></div> <!></div>');function Lf(t,e){Ae(e,!0);let r=G(null),a=G(null);var i=If(),o=u(i),s=u(o),l=u(s),v=d(l),f=T(v),x=d(s,2);{var g=E=>{var C=Pf(),q=T(C,!0);O(U=>m(q,U),[()=>P("queue.clear")]),M("click",C,()=>c.clearQueue()),h(E,C)};Q(x,E=>{c.queue.length>1&&E(g)})}var k=d(o,2);{var _=E=>{var C=Tf(),q=u(C);Gs(q,{size:26,class:"opacity-40"});var U=d(q,2),X=T(U,!0);O(B=>m(X,B),[()=>P("queue.empty")]),h(E,C)},S=E=>{var C=Mf();Re(C,23,()=>c.queue,(q,U)=>q.id+":"+U,(q,U,X)=>{const B=J(()=>c.currentIndex===n(X));var W=Nf(),H=u(W);Ic(H,{size:13,class:"text-[var(--text-muted)]/45 cursor-grab shrink-0"});var F=d(H,2);nt(F,{class:"track-cover !w-9 !h-9 !rounded-md",get src(){return n(U).cover_url},alt:""});var D=d(F,2),V=u(D),Z=T(V,!0),Y=d(V,2),pe=T(Y,!0),de=d(D,2);{var oe=Te=>{var ce=Af();h(Te,ce)},re=Te=>{var ce=zf(),be=T(ce,!0);O(me=>m(be,me),[()=>zt(n(U).duration_secs)]),h(Te,ce)};Q(de,Te=>{n(B)&&c.isPlaying?Te(oe):Te(re,-1)})}var ae=d(de,2),ge=u(ae);Cr(ge,{size:13}),O(()=>{ee(W,1,`group flex items-center gap-2 px-2.5 py-2 border-b border-[var(--border)] last:border-b-0 cursor-pointer transition-colors ${n(B)?"bg-[var(--accent-soft)]":"hover:bg-[var(--surface-hover)]"} ${n(a)===n(X)?"border-t-2 border-t-[var(--accent-bright)]":""}`,"svelte-1567ky0"),ee(V,1,`track-title !text-[12.5px] ${n(B)?"!text-[var(--accent-bright)] !font-bold":""}`,"svelte-1567ky0"),m(Z,n(U).title),m(pe,n(U).artist)}),_t("dragstart",W,()=>z(r,n(X),!0)),_t("dragover",W,Te=>{Te.preventDefault(),z(a,n(X),!0)}),_t("dragleave",W,()=>{n(a)===n(X)&&z(a,null)}),_t("drop",W,()=>{n(r)!=null&&c.moveQueueItem(n(r),n(X)),z(r,null),z(a,null)}),_t("dragend",W,()=>{z(r,null),z(a,null)}),M("click",W,()=>void c.playIndex(n(X))),M("keydown",W,Te=>Te.key==="Enter"&&void c.playIndex(n(X))),M("click",ae,Te=>{Te.stopPropagation(),c.removeQueueIndex(n(X))}),h(q,W)}),h(E,C)};Q(k,E=>{c.queue.length===0?E(_):E(S,-1)})}var I=d(k,2),$=u(I),y=u($),b=T(y,!0),w=d(y,2),A=T(w,!0),N=d($,2),R=T(N),p=d(I,2);{var j=E=>{var C=Cf(),q=K(C),U=T(q,!0),X=d(q,2);Re(X,21,()=>c.suggestedTracks,B=>B.id,(B,W)=>{var H=Ef(),F=u(H);nt(F,{class:"track-cover !w-9 !h-9 !rounded-md",get src(){return n(W).cover_url},alt:""});var D=d(F,2),V=u(D),Z=T(V,!0),Y=d(V,2),pe=T(Y,!0),de=d(D,2),oe=u(de);yr(oe,{size:15}),O(()=>{m(Z,n(W).title),m(pe,n(W).artist)}),M("click",de,()=>c.enqueueTrack(n(W))),h(B,H)}),O(B=>m(U,B),[()=>P("queue.suggestions")]),h(E,C)};Q(p,E=>{c.suggestedTracks.length>0&&E(j)})}O((E,C,q)=>{m(l,`${E??""} `),m(f,`· ${c.queue.length??""}`),m(b,C),m(A,q),le(N,"aria-checked",c.autoplayRelated),ee(N,1,`w-10 h-6 rounded-full p-0.5 transition-colors shrink-0 ${c.autoplayRelated?"bg-[var(--accent)]":"bg-white/15"}`,"svelte-1567ky0"),ee(R,1,`block w-5 h-5 rounded-full bg-white shadow transition-transform ${c.autoplayRelated?"translate-x-4":"translate-x-0"}`,"svelte-1567ky0")},[()=>P("queue.title"),()=>P("queue.autoplay"),()=>P("queue.autoplay_desc")]),M("click",N,()=>c.setAutoplay(!c.autoplayRelated)),ot(1,i,()=>wr,()=>({duration:180})),h(t,i),ze()}Ve(["click","keydown"]);var Rf=L('<aside class="w-full md:w-[388px] lg:w-[360px] xl:w-[388px] shrink-0 h-full flex flex-col border-l max-md:border-l-0 border-[var(--border)] bg-[var(--bg-sidebar)] overflow-hidden relative select-none" style="background-image: linear-gradient(180deg, rgba(var(--ambient-rgb), 0.22) 0%, rgba(var(--ambient-rgb), 0.08) 52%, transparent 100%);"><div class="shrink-0 flex items-center justify-between gap-3 px-5 pt-4 pb-2"><button class="icon-btn shrink-0 text-[var(--text-muted)] hover:text-[var(--text)]"><!> <!></button> <div class="min-w-0 flex flex-col leading-tight text-right"><span class="text-[11.5px] font-medium text-[var(--text-muted)]"> </span> <span class="text-[13px] font-semibold truncate"> </span></div></div> <div><!></div> <nav class="shrink-0 grid grid-cols-3 gap-1.5 px-3 py-2.5 border-t border-[var(--border)] bg-[var(--bg-sidebar)]" style="padding-bottom: max(0.625rem, env(safe-area-inset-bottom));"><button><!> </button> <button><!> </button> <button><!> </button></nav></aside>');function jf(t,e){Ae(e,!0);let r=G(null),a;function i(B){if(window.clearInterval(a),a=void 0,!B){z(r,null);return}z(r,B*60),c.showToast(`Sleep timer set for ${B} min`),a=window.setInterval(()=>{n(r)!=null&&(z(r,n(r)-1),n(r)<=0&&(window.clearInterval(a),a=void 0,z(r,null),c.isPlaying&&c.togglePlay(),c.showToast("Sleep timer ended — playback paused")))},1e3)}var o=Rf(),s=u(o),l=u(s),v=u(l);Tc(v,{size:20,class:"md:hidden"});var f=d(v,2);An(f,{size:19,class:"hidden md:block"});var x=d(l,2),g=u(x),k=T(g,!0),_=d(g,2),S=T(_,!0),I=d(s,2),$=u(I);{var y=B=>{$f(B,{get sleepRemainingSecs(){return n(r)},onSetSleepTimer:i})},b=B=>{ff(B,{})},w=B=>{Lf(B,{})};Q($,B=>{c.sidebarTab==="player"?B(y):c.sidebarTab==="lyrics"?B(b,1):B(w,-1)})}var A=d(I,2),N=u(A),R=u(N);ea(R,{size:15});var p=d(R),j=d(N,2),E=u(j);qc(E,{size:15});var C=d(E),q=d(j,2),U=u(q);Gs(U,{size:15});var X=d(U);O((B,W,H,F,D,V)=>{le(l,"title",B),m(k,W),m(S,H),ee(I,1,`flex-1 min-h-0 px-7 pb-4 flex flex-col ${c.sidebarTab==="lyrics"?"overflow-hidden":"overflow-y-auto scroll-y"}`),ee(N,1,`flex items-center justify-center gap-1.5 py-2 rounded-xl text-[13px] font-semibold border transition-colors ${c.sidebarTab==="player"?"bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]":"border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]"}`),m(p,` ${F??""}`),ee(j,1,`flex items-center justify-center gap-1.5 py-2 rounded-xl text-[13px] font-semibold border transition-colors ${c.sidebarTab==="lyrics"?"bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]":"border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]"}`),m(C,` ${D??""}`),ee(q,1,`flex items-center justify-center gap-1.5 py-2 rounded-xl text-[13px] font-semibold border transition-colors ${c.sidebarTab==="queue"?"bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]":"border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]"}`),m(X,` ${V??""}`)},[()=>P("player.collapse"),()=>P("player.playing_from"),()=>c.sourceName==="Not playing"?P("player.not_playing"):c.sourceName,()=>P("player.tab_player"),()=>P("player.tab_lyrics"),()=>P("player.tab_queue")]),M("click",l,()=>{c.sidebarOpen=!1,c.persistSession()}),M("click",N,()=>c.sidebarTab="player"),M("click",j,()=>c.sidebarTab="lyrics"),M("click",q,()=>c.sidebarTab="queue"),h(t,o),ze()}Ve(["click"]);var qf=L('<div class="qr-box w-[184px] h-[184px] p-2.5 rounded-2xl bg-white shadow-[0_10px_28px_rgba(0,0,0,0.36)] svelte-1cce07c"></div>'),Ri=L("<!> ",1),Of=L('<div class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] p-4 flex flex-col items-center gap-3.5"><span class="text-[11.5px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)]"> </span> <!> <div class="w-full flex items-center gap-2 bg-black/25 border border-[var(--border)] rounded-xl px-3 py-2"><code class="flex-1 min-w-0 text-[12px] text-[var(--text)]/90 truncate select-all"> </code> <button type="button"><!></button></div></div>');function ro(t,e){Ae(e,!0);let r=et(e,"qrSvg",3,null),a=G(!1),i=null;async function o(){if(e.value)try{await navigator.clipboard.writeText(e.value),c.showToast(e.copyToast),z(a,!0),i&&window.clearTimeout(i),i=window.setTimeout(()=>{z(a,!1)},1800)}catch{c.showToast(P("spin.copy_failed"))}}var s=Of(),l=u(s),v=T(l,!0),f=d(l,2);{var x=b=>{var w=qf();Xl(w,r,!0),h(b,w)};Q(f,b=>{r()&&b(x)})}var g=d(f,2),k=u(g),_=T(k,!0),S=d(k,2),I=u(S);{var $=b=>{var w=Ri(),A=K(w);Qs(A,{size:13});var N=d(A);O(R=>m(N,` ${R??""}`),[()=>P("spin.copied")]),h(b,w)},y=b=>{var w=Ri(),A=K(w);Mc(A,{size:13});var N=d(A);O(R=>m(N,` ${R??""}`),[()=>P("spin.copy")]),h(b,w)};Q(I,b=>{n(a)?b($):b(y,-1)})}O(()=>{m(v,e.kicker),m(_,e.value),ee(S,1,`inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg text-[12px] font-semibold shrink-0 transition-all active:scale-95 ${n(a)?"bg-emerald-500/20 text-emerald-300 border border-emerald-500/30":"bg-white/[0.08] hover:bg-white/[0.15] text-[var(--text)] border border-white/10"}`)}),M("click",S,()=>void o()),h(t,s),ze()}Ve(["click"]);var Df=L('<div class="space-y-4"><div class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] divide-y divide-[var(--border)]"><div class="p-3.5 flex items-center justify-between gap-3"><div class="min-w-0"><div class="text-[14px] font-semibold"> </div> <div class="text-[12px] text-[var(--text-muted)]"> </div></div> <button type="button" role="switch"><span></span></button></div> <div class="p-3.5 flex items-center justify-between gap-3"><div class="min-w-0"><div class="text-[14px] font-semibold"> </div> <div class="text-[12px] text-[var(--text-muted)]"> </div></div> <button type="button" role="switch"><span></span></button></div> <div class="px-3.5 py-2.5 flex items-center justify-between text-[12.5px]"><span class="text-[var(--text-muted)] font-medium"> </span> <span> </span></div></div> <!></div>');function Ff(t,e){Ae(e,!0);let r=G(!1);async function a(){if(!Ue()||n(r))return;z(r,!0);const B=!c.spinStatus.local_active;try{c.spinStatus=B?await Ne.spinStartLocal():await Ne.spinStopLocal()}catch(W){c.showToast(String(W))}finally{z(r,!1)}}async function i(){if(Ue())try{c.spinStatus=await Ne.spinSetAllowControls(!c.spinStatus.allow_controls)}catch(B){c.showToast(String(B))}}var o=Df(),s=u(o),l=u(s),v=u(l),f=u(v),x=T(f,!0),g=d(f,2),k=T(g,!0),_=d(v,2),S=T(_),I=d(l,2),$=u(I),y=u($),b=T(y,!0),w=d(y,2),A=T(w,!0),N=d($,2),R=T(N),p=d(I,2),j=u(p),E=T(j,!0),C=d(j,2),q=T(C,!0),U=d(s,2);{var X=B=>{{let W=J(()=>P("spin.scan_to_join")),H=J(()=>P("spin.link_copied"));ro(B,{get kicker(){return n(W)},get qrSvg(){return c.spinStatus.local_qr_svg},get value(){return c.spinStatus.local_url},get copyToast(){return n(H)}})}};Q(U,B=>{c.spinStatus.local_active&&B(X)})}O((B,W,H,F,D,V,Z,Y)=>{m(x,B),m(k,W),le(_,"aria-checked",c.spinStatus.local_active),le(_,"aria-label",H),_.disabled=n(r),ee(_,1,`relative w-11 h-6 rounded-full transition-colors shrink-0 disabled:opacity-50 ${c.spinStatus.local_active?"bg-[var(--accent)]":"bg-white/15"}`),ee(S,1,`absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow transition-transform ${c.spinStatus.local_active?"translate-x-5":""}`),m(b,F),m(A,D),le(N,"aria-checked",c.spinStatus.allow_controls),le(N,"aria-label",V),ee(N,1,`relative w-11 h-6 rounded-full transition-colors shrink-0 ${c.spinStatus.allow_controls?"bg-[var(--accent)]":"bg-white/15"}`),ee(R,1,`absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow transition-transform ${c.spinStatus.allow_controls?"translate-x-5":""}`),m(E,Z),ee(C,1,`font-semibold numeric ${c.spinStatus.local_active?"text-[var(--accent-bright)]":"text-[var(--text-muted)]"}`),m(q,Y)},[()=>P("spin.local_enable"),()=>P("spin.local_enable_sub"),()=>P("spin.local_enable"),()=>P("spin.allow_controls"),()=>P("spin.allow_controls_sub"),()=>P("spin.allow_controls"),()=>P("spin.status"),()=>c.spinStatus.local_active?P("spin.status_active",{guests:c.spinStatus.guest_count,added:c.spinStatus.added_count}):P("spin.status_inactive")]),M("click",_,()=>void a()),M("click",N,()=>void i()),h(t,o),ze()}Ve(["click"]);var ji=L("<!> ",1),Hf=L('<div class="space-y-4"><div class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] p-3.5 flex items-center justify-between gap-3"><div class="min-w-0"><div class="text-[14px] font-semibold"> </div> <div class="text-[12px] text-[var(--text-muted)]"> </div></div> <button type="button" role="switch"><span></span></button></div> <!> <form class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] p-4 space-y-3"><div><div class="text-[14px] font-semibold"> </div> <p class="text-[12px] text-[var(--text-muted)] mt-0.5"> </p></div> <div class="flex items-center gap-2"><input class="flex-1 min-w-0 px-3 py-2 rounded-xl bg-black/25 border border-[var(--border-strong)] text-[13px] outline-none focus:border-[var(--accent)] transition-colors" type="text"/> <button type="submit"><!></button></div></form></div>');function Vf(t,e){Ae(e,!0);let r=G(""),a=G(!1),i=J(()=>c.spinStatus.remote_role==="host"),o=J(()=>c.spinStatus.remote_role==="guest");async function s(){if(!(!Ue()||n(a))){z(a,!0);try{c.spinStatus=n(i)?await Ne.spinStopRemote():await Ne.spinStartRemoteHost()}catch(W){c.showToast(String(W))}finally{z(a,!1)}}}async function l(){if(!(!Ue()||n(a))){z(a,!0);try{n(o)?c.spinStatus=await Ne.spinStopRemote():n(r).trim()&&(c.spinStatus=await Ne.spinJoinRemote(n(r).trim()),c.showToast(P("spin.connecting")))}catch(W){c.showToast(String(W))}finally{z(a,!1)}}}var v=Hf(),f=u(v),x=u(f),g=u(x),k=T(g,!0),_=d(g,2),S=T(_,!0),I=d(x,2),$=T(I),y=d(f,2);{var b=W=>{{let H=J(()=>P("spin.active_invite")),F=J(()=>P("spin.invite_copied"));ro(W,{get kicker(){return n(H)},get qrSvg(){return c.spinStatus.remote_qr_svg},get value(){return c.spinStatus.remote_ticket},get copyToast(){return n(F)}})}};Q(y,W=>{n(i)&&c.spinStatus.remote_ticket&&W(b)})}var w=d(y,2),A=u(w),N=u(A),R=T(N,!0),p=d(N,2),j=T(p,!0),E=d(A,2),C=u(E),q=d(C,2),U=u(q);{var X=W=>{var H=ji(),F=K(H);jc(F,{size:14});var D=d(F);O(V=>m(D,` ${V??""}`),[()=>P("spin.disconnect_btn")]),h(W,H)},B=W=>{var H=ji(),F=K(H);Rc(F,{size:14});var D=d(F);O(V=>m(D,` ${V??""}`),[()=>P("spin.join_btn")]),h(W,H)};Q(U,W=>{n(o)?W(X):W(B,-1)})}O((W,H,F,D,V,Z,Y)=>{m(k,W),m(S,H),le(I,"aria-checked",n(i)),le(I,"aria-label",F),I.disabled=n(a),ee(I,1,`relative w-11 h-6 rounded-full transition-colors shrink-0 disabled:opacity-50 ${n(i)?"bg-[var(--accent)]":"bg-white/15"}`),ee($,1,`absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow transition-transform ${n(i)?"translate-x-5":""}`),m(R,D),m(j,V),C.disabled=n(o)||n(a),le(C,"placeholder",Z),ee(q,1,`pill-btn shrink-0 !px-4 !py-2 ${n(o)?"hover:!text-red-400":"primary"}`),q.disabled=Y},[()=>P("spin.host_enable"),()=>P("spin.host_enable_sub"),()=>P("spin.host_enable"),()=>P("spin.join_title"),()=>P("spin.join_sub"),()=>P("spin.join_placeholder"),()=>n(a)||!n(o)&&!n(r).trim()]),M("click",I,()=>void s()),_t("submit",w,W=>{W.preventDefault(),l()}),qa(C,()=>n(r),W=>z(r,W)),h(t,v),ze()}Ve(["click"]);var Qf=L('<span class="inline-flex items-center gap-1.5 text-[11px] font-bold px-2.5 py-0.5 rounded-full bg-emerald-500/15 text-emerald-300 border border-emerald-500/25"><span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span> </span>'),Uf=L('<div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-md" role="presentation"><div role="dialog" aria-modal="true" class="w-[min(480px,calc(100vw-32px))] max-h-[calc(100vh-48px)] overflow-y-auto rounded-[18px] bg-[var(--bg-elevated)] text-[var(--text)] border border-[var(--border-strong)] shadow-[0_24px_64px_rgba(0,0,0,0.62)]"><div class="flex items-center justify-between px-6 py-4 border-b border-[var(--border)]"><div class="flex items-center gap-2.5"><div class="w-8 h-8 rounded-xl bg-[var(--accent-soft)] text-[var(--accent-bright)] flex items-center justify-center"><!></div> <h2 class="text-[18px] font-extrabold tracking-tight"> </h2> <!></div> <button type="button" class="icon-btn"><!></button></div> <div class="p-6 space-y-5"><div class="grid grid-cols-2 gap-1 p-1 rounded-xl bg-black/25 border border-[var(--border)]"><button type="button"><!> </button> <button type="button"><!> </button></div> <!></div></div></div>');function Bf(t,e){Ae(e,!0);let r=G("local"),a=J(()=>c.spinStatus.local_active||c.spinStatus.remote_role!=="idle");function i(v){c.spinOpen&&v.key==="Escape"&&(v.preventDefault(),c.spinOpen=!1)}var o=se();_t("keydown",ga,i);var s=K(o);{var l=v=>{var f=Uf(),x=u(f),g=u(x),k=u(g),_=u(k),S=u(_);$a(S,{size:17});var I=d(_,2),$=T(I,!0),y=d(I,2);{var b=H=>{var F=Qf(),D=d(u(F));O(V=>m(D,` ${V??""}`),[()=>P("spin.live")]),h(H,F)};Q(y,H=>{n(a)&&H(b)})}var w=d(k,2),A=u(w);Cr(A,{size:17});var N=d(g,2),R=u(N),p=u(R),j=u(p);Yc(j,{size:15});var E=d(j),C=d(p,2),q=u(C);Bs(q,{size:15});var U=d(q),X=d(R,2);{var B=H=>{Ff(H,{})},W=H=>{Vf(H,{})};Q(X,H=>{n(r)==="local"?H(B):H(W,-1)})}O((H,F,D,V,Z)=>{le(x,"aria-label",H),m($,F),le(w,"title",D),ee(p,1,`inline-flex items-center justify-center gap-2 py-2 px-3 rounded-lg text-[13px] font-semibold transition-all ${n(r)==="local"?"bg-[var(--accent)] text-white shadow-sm":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),m(E,` ${V??""}`),ee(C,1,`inline-flex items-center justify-center gap-2 py-2 px-3 rounded-lg text-[13px] font-semibold transition-all ${n(r)==="remote"?"bg-[var(--accent)] text-white shadow-sm":"text-[var(--text-muted)] hover:text-[var(--text)]"}`),m(U,` ${Z??""}`)},[()=>P("spin.title"),()=>P("spin.title"),()=>P("spin.close"),()=>P("spin.tab_local"),()=>P("spin.tab_remote")]),M("click",f,H=>{H.target===H.currentTarget&&(c.spinOpen=!1)}),M("click",w,()=>c.spinOpen=!1),M("click",p,()=>z(r,"local")),M("click",C,()=>z(r,"remote")),ot(3,x,()=>Ir,()=>({start:.95,duration:190,easing:At})),ot(3,f,()=>wr,()=>({duration:160})),h(v,f)};Q(s,v=>{c.spinOpen&&v(l)})}h(t,o),ze()}Ve(["click"]);var qi=L("<div><!></div>"),Wf=L('<div class="lg:hidden fixed inset-0 z-40 bg-black/55 backdrop-blur-sm" role="presentation"></div> <div class="h-full shrink-0 max-lg:fixed max-lg:inset-y-0 max-lg:right-0 max-lg:z-50 max-md:inset-0"><!></div>',1),Gf=L('<div class="fixed bottom-5 max-md:bottom-[66px] left-1/2 -translate-x-1/2 z-30 w-[min(560px,calc(100vw-24px))] flex items-center gap-2 sm:gap-2.5 px-3 sm:px-3.5 py-2 rounded-full bg-[var(--bg-elevated)]/95 backdrop-blur-xl border border-[var(--border-strong)] shadow-2xl"><button class="flex-1 min-w-0 flex items-center gap-2.5 text-left"><!> <div class="track-meta"><span class="track-title"> </span> <span class="track-sub"> </span></div></button> <button title="Favorite"><!></button> <button class="icon-btn hidden sm:inline-flex" title="Previous"><!></button> <button class="play-circle !w-10 !h-10" title="Play / Pause"><!></button> <button class="icon-btn" title="Next"><!></button></div>'),Yf=L('<div class="px-4 py-2 rounded-full text-[13px] font-medium bg-[var(--bg-elevated)]/95 backdrop-blur-md border border-[var(--border-strong)] shadow-xl"> </div>'),Kf=L('<div class="fixed bottom-5 left-1/2 -translate-x-1/2 z-50 flex flex-col items-center gap-2 pointer-events-none" aria-live="polite"></div>'),Jf=L('<div class="h-screen w-screen overflow-hidden flex flex-col bg-[var(--bg)] text-[var(--text)]"><!> <div class="flex-1 min-h-0 flex overflow-hidden relative"><!> <main class="flex-1 min-w-0 h-full overflow-y-auto overflow-x-hidden ambient-surface scroll-y max-md:pb-24"><!></main> <!></div> <!> <!> <!> <!> <!></div>');function Xf(t,e){Ae(e,!0),Sa(()=>{c.bootstrap()});let r=J(()=>c.activeView.kind==="artist"?`artist:${c.activeView.artistId}`:c.activeView.kind==="album"?`album:${c.activeView.id}`:c.activeView.kind);$t(()=>{const w=c.ambientRgb;document.documentElement.style.setProperty("--ambient-rgb",w?`${w[0]}, ${w[1]}, ${w[2]}`:"33, 144, 164")});var a=Jf(),i=u(a);_v(i);var o=d(i,2),s=u(o);Fv(s,{});var l=d(s,2),v=u(l);Hn(v,()=>n(r),w=>{var A=qi(),N=u(A);{var R=q=>{jv(q,{})},p=q=>{tv(q,{})},j=q=>{ju(q,{})},E=q=>{ru(q,{get artistId(){return c.activeView.artistId},get artistName(){return c.activeView.artistName}})},C=q=>{Sd(q,{get id(){return c.activeView.id},get title(){return c.activeView.title},get coverUrl(){return c.activeView.coverUrl},get isPlaylist(){return c.activeView.isPlaylist}})};Q(N,q=>{c.activeView.kind==="home"?q(R):c.activeView.kind==="explore"?q(p,1):c.activeView.kind==="collection"?q(j,2):c.activeView.kind==="artist"?q(E,3):c.activeView.kind==="album"&&q(C,4)})}ot(1,A,()=>pr,()=>({y:10,duration:220,easing:At})),h(w,A)});var f=d(l,2);{var x=w=>{var A=Wf(),N=K(A),R=d(N,2),p=u(R);jf(p,{}),M("click",N,()=>{c.sidebarOpen=!1,c.persistSession()}),ot(3,N,()=>wr,()=>({duration:160})),ot(3,R,()=>pr,()=>({x:28,duration:220,easing:At})),h(w,A)};Q(f,w=>{c.sidebarOpen&&w(x)})}var g=d(o,2);{var k=w=>{var A=Gf(),N=u(A),R=u(N);nt(R,{class:"w-10 h-10 !rounded-[8px] object-cover shrink-0",get src(){return c.currentTrack.cover_url},alt:""});var p=d(R,2),j=u(p),E=T(j,!0),C=d(j,2),q=T(C,!0),U=d(N,2);let X;var B=u(U);{let de=J(()=>c.isFavorite(c.currentTrack.id)?"currentColor":"none");gt(B,{size:17,get fill(){return n(de)}})}var W=d(U,2),H=u(W);Gn(H,{size:17,fill:"currentColor"});var F=d(W,2),D=u(F);{var V=de=>{Wn(de,{size:17,fill:"currentColor"})},Z=de=>{bt(de,{size:17,fill:"currentColor",class:"translate-x-[1px]"})};Q(D,de=>{c.isPlaying?de(V):de(Z,-1)})}var Y=d(F,2),pe=u(Y);Yn(pe,{size:17,fill:"currentColor"}),O(de=>{m(E,c.currentTrack.title),m(q,c.currentTrack.artist),X=ee(U,1,"icon-btn",null,X,{"fav-active":de})},[()=>c.isFavorite(c.currentTrack.id)]),M("click",N,()=>c.sidebarOpen=!0),M("click",U,()=>c.currentTrack&&c.toggleFavorite(c.currentTrack)),M("click",W,()=>void c.prev()),M("click",F,()=>void c.togglePlay()),M("click",Y,()=>void c.next()),ot(3,A,()=>pr,()=>({y:24,duration:240,easing:At})),h(w,A)};Q(g,w=>{!c.sidebarOpen&&c.currentTrack&&w(k)})}var _=d(g,2);{var S=w=>{var A=qi(),N=u(A);Xv(N,{}),ot(3,A,()=>wr,()=>({duration:220})),h(w,A)};Q(_,w=>{c.partyMode&&w(S)})}var I=d(_,2);rf(I,{});var $=d(I,2);Bf($,{});var y=d($,2);{var b=w=>{var A=Kf();Re(A,21,()=>c.toasts,N=>N.id,(N,R)=>{var p=Yf(),j=T(p,!0);O(()=>m(j,n(R).message)),ot(3,p,()=>pr,()=>({y:14,duration:200,easing:At})),h(N,p)}),h(w,A)};Q(y,w=>{c.toasts.length>0&&w(b)})}h(t,a),ze()}Ve(["click"]);Ul(Xf,{target:document.getElementById("app")});
