let n;const e=new Array(32).fill(void 0);function t(n){return e[n]}e.push(void 0,null,!0,!1);let r=0,o=null;function i(){return null!==o&&o.buffer===n.memory.buffer||(o=new Uint8Array(n.memory.buffer)),o}let a=new TextEncoder("utf-8");const s="function"==typeof a.encodeInto?function(n,e){return a.encodeInto(n,e)}:function(n,e){const t=a.encode(n);return e.set(t),{read:n.length,written:t.length}};let c=null;function u(){return null!==c&&c.buffer===n.memory.buffer||(c=new Int32Array(n.memory.buffer)),c}let f=e.length;function l(n){const r=t(n);return function(n){n<36||(e[n]=f,f=n)}(n),r}let b=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});function w(n,e){return b.decode(i().subarray(n,n+e))}function g(n){f===e.length&&e.push(e.length+1);const t=f;return f=e[t],e[t]=n,t}async function d(e){void 0===e&&(e=import.meta.url.replace(/\.js$/,"_bg.wasm"));const o={wbg:{}};o.wbg.__wbindgen_string_get=function(e,o){const c=t(o);var f="string"==typeof c?c:void 0,l=null==f?0:function(n,e,t){if(void 0===t){const t=a.encode(n),o=e(t.length);return i().subarray(o,o+t.length).set(t),r=t.length,o}let o=n.length,c=e(o);const u=i();let f=0;for(;f<o;f++){const e=n.charCodeAt(f);if(e>127)break;u[c+f]=e}if(f!==o){0!==f&&(n=n.slice(f)),c=t(c,o,o=f+3*n.length);const e=i().subarray(c+f,c+o);f+=s(n,e).written}return r=f,c}(f,n.__wbindgen_malloc,n.__wbindgen_realloc),b=r;u()[e/4+1]=b,u()[e/4+0]=l},o.wbg.__wbindgen_object_drop_ref=function(n){l(n)},o.wbg.__wbindgen_string_new=function(n,e){return g(w(n,e))},o.wbg.__wbindgen_throw=function(n,e){throw new Error(w(n,e))},("string"==typeof e||"function"==typeof Request&&e instanceof Request||"function"==typeof URL&&e instanceof URL)&&(e=fetch(e));const{instance:c,module:f}=await async function(n,e){if("function"==typeof Response&&n instanceof Response){if("function"==typeof WebAssembly.instantiateStreaming)try{return await WebAssembly.instantiateStreaming(n,e)}catch(e){if("application/wasm"==n.headers.get("Content-Type"))throw e;console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e)}const t=await n.arrayBuffer();return await WebAssembly.instantiate(t,e)}{const t=await WebAssembly.instantiate(n,e);return t instanceof WebAssembly.Instance?{instance:t,module:n}:t}}(await e,o);return n=c.exports,d.__wbindgen_wasm_module=f,n}b.decode();var y=Object.freeze({__proto__:null,parse:function(e){return l(n.parse(g(e)))},default:d});export default async()=>(await d("parser-parser-core.wasm"),y);