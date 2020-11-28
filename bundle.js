function t(){}function e(t){return t()}function n(){return Object.create(null)}function o(t){t.forEach(e)}function r(t){return"function"==typeof t}function i(t,e){return t!=t?e==e:t!==e||t&&"object"==typeof t||"function"==typeof t}function c(t,e){t.appendChild(e)}function l(t,e,n){t.insertBefore(e,n||null)}function s(t){t.parentNode.removeChild(t)}function a(t){return document.createElement(t)}function u(t){return document.createTextNode(t)}function d(){return u(" ")}function f(){return u("")}function m(t,e,n,o){return t.addEventListener(e,n,o),()=>t.removeEventListener(e,n,o)}function p(t,e,n){null==n?t.removeAttribute(e):t.getAttribute(e)!==n&&t.setAttribute(e,n)}let h,g;function b(){if(void 0===h){h=!1;try{"undefined"!=typeof window&&window.parent&&window.parent.document}catch(t){h=!0}}return h}function v(t,e,n){t.classList[n?"add":"remove"](e)}function y(t){g=t}function $(){if(!g)throw new Error("Function called outside component initialization");return g}function x(){const t=$();return(e,n)=>{const o=t.$$.callbacks[e];if(o){const r=function(t,e){const n=document.createEvent("CustomEvent");return n.initCustomEvent(t,!1,!1,e),n}(e,n);o.slice().forEach(e=>{e.call(t,r)})}}}const w=[],k=[],C=[],E=[],_=Promise.resolve();let N=!1;function A(t){C.push(t)}let H=!1;const I=new Set;function T(){if(!H){H=!0;do{for(let t=0;t<w.length;t+=1){const e=w[t];y(e),M(e.$$)}for(w.length=0;k.length;)k.pop()();for(let t=0;t<C.length;t+=1){const e=C[t];I.has(e)||(I.add(e),e())}C.length=0}while(w.length);for(;E.length;)E.pop()();N=!1,H=!1,I.clear()}}function M(t){if(null!==t.fragment){t.update(),o(t.before_update);const e=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,e),t.after_update.forEach(A)}}const z=new Set;let j;function B(t,e){t&&t.i&&(z.delete(t),t.i(e))}function L(t,e,n,o){if(t&&t.o){if(z.has(t))return;z.add(t),j.c.push(()=>{z.delete(t),o&&(n&&t.d(1),o())}),t.o(e)}}function S(t,e){const n=e.token={};function r(t,r,i,c){if(e.token!==n)return;e.resolved=c;let l=e.ctx;void 0!==i&&(l=l.slice(),l[i]=c);const s=t&&(e.current=t)(l);let a=!1;e.block&&(e.blocks?e.blocks.forEach((t,n)=>{n!==r&&t&&(j={r:0,c:[],p:j},L(t,1,1,()=>{e.blocks[n]=null}),j.r||o(j.c),j=j.p)}):e.block.d(1),s.c(),B(s,1),s.m(e.mount(),e.anchor),a=!0),e.block=s,e.blocks&&(e.blocks[r]=s),a&&T()}if((i=t)&&"object"==typeof i&&"function"==typeof i.then){const n=$();if(t.then(t=>{y(n),r(e.then,1,e.value,t),y(null)},t=>{y(n),r(e.catch,2,e.error,t),y(null)}),e.current!==e.pending)return r(e.pending,0),!0}else{if(e.current!==e.then)return r(e.then,1,e.value,t),!0;e.resolved=t}var i}function P(t){t&&t.c()}function W(t,n,i){const{fragment:c,on_mount:l,on_destroy:s,after_update:a}=t.$$;c&&c.m(n,i),A(()=>{const n=l.map(e).filter(r);s?s.push(...n):o(n),t.$$.on_mount=[]}),a.forEach(A)}function O(t,e){const n=t.$$;null!==n.fragment&&(o(n.on_destroy),n.fragment&&n.fragment.d(e),n.on_destroy=n.fragment=null,n.ctx=[])}function V(t,e){-1===t.$$.dirty[0]&&(w.push(t),N||(N=!0,_.then(T)),t.$$.dirty.fill(0)),t.$$.dirty[e/31|0]|=1<<e%31}function K(e,r,i,c,l,a,u=[-1]){const d=g;y(e);const f=r.props||{},m=e.$$={fragment:null,ctx:null,props:a,update:t,not_equal:l,bound:n(),on_mount:[],on_destroy:[],before_update:[],after_update:[],context:new Map(d?d.$$.context:[]),callbacks:n(),dirty:u};let p=!1;if(m.ctx=i?i(e,f,(t,n,...o)=>{const r=o.length?o[0]:n;return m.ctx&&l(m.ctx[t],m.ctx[t]=r)&&(m.bound[t]&&m.bound[t](r),p&&V(e,t)),n}):[],m.update(),p=!0,o(m.before_update),m.fragment=!!c&&c(m.ctx),r.target){if(r.hydrate){const t=function(t){return Array.from(t.childNodes)}(r.target);m.fragment&&m.fragment.l(t),t.forEach(s)}else m.fragment&&m.fragment.c();r.intro&&B(e.$$.fragment),W(e,r.target,r.anchor),T()}y(d)}class J{$destroy(){O(this,1),this.$destroy=t}$on(t,e){const n=this.$$.callbacks[t]||(this.$$.callbacks[t]=[]);return n.push(e),()=>{const t=n.indexOf(e);-1!==t&&n.splice(t,1)}}$set(){}}function q(e){let n,o,r;return{c(){n=a("div"),o=a("textarea"),o.readOnly=!0,p(o,"class","svelte-1evpbm0"),p(n,"class","codemirror-container svelte-1evpbm0"),A(()=>e[18].call(n)),v(n,"flex",e[0])},m(t,i){l(t,n,i),c(n,o),e[17](o),r=function(t,e){const n=getComputedStyle(t),o=(parseInt(n.zIndex)||0)-1;"static"===n.position&&(t.style.position="relative");const r=a("iframe");r.setAttribute("style",`display: block; position: absolute; top: 0; left: 0; width: 100%; height: 100%; overflow: hidden; border: 0; opacity: 0; pointer-events: none; z-index: ${o};`),r.setAttribute("aria-hidden","true"),r.tabIndex=-1;const i=b();let l;return i?(r.src="data:text/html,<script>onresize=function(){parent.postMessage(0,'*')}<\/script>",l=m(window,"message",t=>{t.source===r.contentWindow&&e()})):(r.src="about:blank",r.onload=()=>{l=m(r.contentWindow,"resize",e)}),c(t,r),()=>{(i||l&&r.contentWindow)&&l(),s(r)}}(n,e[18].bind(n))},p(t,[e]){1&e&&v(n,"flex",t[0])},i:t,o:t,d(t){t&&s(n),e[17](null),r()}}}function F(t){return new Promise(e=>setTimeout(e,t))}function U(t,e,n){const o=x();let r,i,c,l,{readonly:s=!1}=e,{flex:a=!1}=e,{lineNumbers:u=!0}=e,{tab:d=!0}=e,{lint:f=null}=e,{mode:m="ebnf"}=e,p="";let h,g=!1,b=!1;var v;v=()=>((async()=>{let t=await import("./codemirror-9ea8e240.js");h=t.default,await async function(t){if(b||!h)return;l&&l.toTextArea();const e={lineNumbers:u,lineWrapping:!0,indentWithTabs:!0,indentUnit:2,tabSize:2,tabIndex:4,value:"",mode:{name:t},readOnly:s,autoCloseBrackets:!0,autoCloseTags:!0,extraKeys:{Enter:"newlineAndIndentContinueMarkdownList","Ctrl-/":"toggleComment","Cmd-/":"toggleComment"},lint:!!f&&{getAnnotations:f,delay:Number.EPSILON},theme:"gruvbox-dark"};d||(e.extraKeys.Tab=d,e.extraKeys["Shift-Tab"]=d),y&&await F(50),b||(n(20,l=h.fromTextArea(c,e)),l.on("change",t=>{if(!g){const e=t.getValue();o("change",{value:e})}}),y&&await F(50),l.refresh(),y=!1)}(m),l&&l.setValue(p||"")})(),()=>{b=!0,l&&l.toTextArea()}),$().$$.on_mount.push(v);let y=!0;return t.$set=t=>{"readonly"in t&&n(4,s=t.readonly),"flex"in t&&n(0,a=t.flex),"lineNumbers"in t&&n(5,u=t.lineNumbers),"tab"in t&&n(6,d=t.tab),"lint"in t&&n(7,f=t.lint),"mode"in t&&n(8,m=t.mode)},t.$$.update=()=>{1048582&t.$$.dirty&&l&&r&&i&&l.refresh()},[a,r,i,c,s,u,d,f,m,async function(t){p=t,g=!0,l&&l.setValue(p),g=!1},function(){return l.getValue()},function(t){if(p=t,l){const{left:e,top:n}=l.getScrollInfo();l.setValue(p=t),l.scrollTo(e,n)}},function(){l.refresh()},function(){l.focus()},function(){return l.getHistory()},function(t){l.setHistory(t)},function(){l&&l.clearHistory()},function(t){k[t?"unshift":"push"](()=>{c=t,n(3,c)})},function(){r=this.offsetWidth,i=this.offsetHeight,n(1,r),n(2,i)}]}class D extends J{constructor(t){var e;super(),document.getElementById("svelte-1evpbm0-style")||((e=a("style")).id="svelte-1evpbm0-style",e.textContent='.codemirror-container.svelte-1evpbm0{position:relative;width:100%;height:100%;border:none;line-height:1.5;overflow:hidden}.codemirror-container.svelte-1evpbm0 .CodeMirror{height:100%;background:transparent;font:400 16px/1.7;font-family:"JetBrains Mono", Consolas, monospace;color:var(--base)}.codemirror-container.flex.svelte-1evpbm0 .CodeMirror{height:auto}.codemirror-container.flex.svelte-1evpbm0 .CodeMirror-lines{padding:0}.codemirror-container.svelte-1evpbm0 .CodeMirror-gutters{padding:0 16px 0 8px;border:none}.codemirror-container.svelte-1evpbm0 .error-loc{position:relative;border-bottom:2px solid #da106e}.codemirror-container.svelte-1evpbm0 .error-line{background-color:rgba(200, 0, 0, 0.05)}textarea.svelte-1evpbm0{visibility:hidden}',c(document.head,e)),K(this,t,U,q,i,{readonly:4,flex:0,lineNumbers:5,tab:6,lint:7,mode:8,set:9,get:10,update:11,resize:12,focus:13,getHistory:14,setHistory:15,clearHistory:16})}get set(){return this.$$.ctx[9]}get get(){return this.$$.ctx[10]}get update(){return this.$$.ctx[11]}get resize(){return this.$$.ctx[12]}get focus(){return this.$$.ctx[13]}get getHistory(){return this.$$.ctx[14]}get setHistory(){return this.$$.ctx[15]}get clearHistory(){return this.$$.ctx[16]}}function G(t){let e,n,o="> "+t[2];return{c(){e=a("p"),n=u(o),p(e,"class","output svelte-oy86wb")},m(t,o){l(t,e,o),c(e,n)},p(t,e){4&e&&o!==(o="> "+t[2])&&function(t,e){e=""+e,t.data!==e&&(t.data=e)}(n,o)},d(t){t&&s(e)}}}function Q(t){let e,n,o,r,i,u,f,m,h={lint:t[5]};o=new D({props:h}),t[7](o),o.$on("change",t[3]);u=new D({props:{mode:"text"}}),t[8](u),u.$on("change",t[4]);let g=""!=t[2]&&G(t);return{c(){e=a("div"),n=a("div"),P(o.$$.fragment),r=d(),i=a("div"),P(u.$$.fragment),f=d(),g&&g.c(),p(n,"class","editor-left svelte-oy86wb"),p(i,"class","editor-right svelte-oy86wb"),p(e,"class","container svelte-oy86wb")},m(t,s){l(t,e,s),c(e,n),W(o,n,null),c(e,r),c(e,i),W(u,i,null),c(e,f),g&&g.m(e,null),m=!0},p(t,[n]){o.$set({});u.$set({}),""!=t[2]?g?g.p(t,n):(g=G(t),g.c(),g.m(e,null)):g&&(g.d(1),g=null)},i(t){m||(B(o.$$.fragment,t),B(u.$$.fragment,t),m=!0)},o(t){L(o.$$.fragment,t),L(u.$$.fragment,t),m=!1},d(n){n&&s(e),t[7](null),O(o),t[8](null),O(u),g&&g.d()}}}function R(t,e,n){let o,r,i,c,{core:l}=e,s="";return t.$set=t=>{"core"in t&&n(6,l=t.core)},[o,r,s,async function(t){i=void 0,n(2,s="");try{i=new l.EbnfParserParser(t.detail.value),c=null,n(2,s=i.check(r.get())?"success":"failure")}catch(t){console.error(t),c={message:t.kind,from:{line:t.span.from.line,ch:t.span.from.ch},to:{line:t.span.to.line,ch:t.span.to.ch}}}},function(t){i&&n(2,s=i.check(t.detail.value)?"success":"failure")},function(){let t=[];return c&&t.push(c),t},l,function(t){k[t?"unshift":"push"](()=>{o=t,n(0,o)})},function(t){k[t?"unshift":"push"](()=>{r=t,n(1,r)})}]}class X extends J{constructor(t){var e;super(),document.getElementById("svelte-oy86wb-style")||((e=a("style")).id="svelte-oy86wb-style",e.textContent='.container.svelte-oy86wb{position:relative;width:100%;height:100%;display:inline-block}.editor-left.svelte-oy86wb{width:50%;height:100%;float:left}.editor-right.svelte-oy86wb{width:50%;height:15%;float:left}.output.svelte-oy86wb{float:left;font-family:"JetBrains Mono", Consolas, monospace;color:#928374;padding-left:24px}',c(document.head,e)),K(this,t,R,Q,i,{core:6})}}function Y(e){let n,o,r=e[2].message+"";return{c(){var t,e,i;n=a("p"),o=u(r),t="color",e="red",n.style.setProperty(t,e,i?"important":"")},m(t,e){l(t,n,e),c(n,o)},i:t,o:t,d(t){t&&s(n)}}}function Z(t){let e,n,o,r={ctx:t,current:null,token:null,pending:nt,then:et,catch:tt,value:1,blocks:[,,,]};return S(n=t[0].default(),r),{c(){e=f(),r.block.c()},m(t,n){l(t,e,n),r.block.m(t,r.anchor=n),r.mount=()=>e.parentNode,r.anchor=e,o=!0},p(e,n){t=e},i(t){o||(B(r.block),o=!0)},o(t){for(let t=0;t<3;t+=1){L(r.blocks[t])}o=!1},d(t){t&&s(e),r.block.d(t),r.token=null,r=null}}}function tt(e){return{c:t,m:t,i:t,o:t,d:t}}function et(t){let e,n;return e=new X({props:{core:t[1]}}),{c(){P(e.$$.fragment)},m(t,o){W(e,t,o),n=!0},i(t){n||(B(e.$$.fragment,t),n=!0)},o(t){L(e.$$.fragment,t),n=!1},d(t){O(e,t)}}}function nt(e){let n;return{c(){n=a("p"),n.textContent="Loading core..."},m(t,e){l(t,n,e)},i:t,o:t,d(t){t&&s(n)}}}function ot(e){let n;return{c(){n=a("p"),n.textContent="Loading module..."},m(t,e){l(t,n,e)},i:t,o:t,d(t){t&&s(n)}}}function rt(t){let e,n,o,r={ctx:t,current:null,token:null,pending:ot,then:Z,catch:Y,value:0,error:2,blocks:[,,,]};return S(n=import("./Cargo-c68fe751.js"),r),{c(){e=f(),r.block.c()},m(t,n){l(t,e,n),r.block.m(t,r.anchor=n),r.mount=()=>e.parentNode,r.anchor=e,o=!0},p(e,[n]){t=e},i(t){o||(B(r.block),o=!0)},o(t){for(let t=0;t<3;t+=1){L(r.blocks[t])}o=!1},d(t){t&&s(e),r.block.d(t),r.token=null,r=null}}}new class extends J{constructor(t){var e;super(),document.getElementById("svelte-1vl3ij0-style")||((e=a("style")).id="svelte-1vl3ij0-style",e.textContent="body{background-color:rgb(40, 40, 40)}",c(document.head,e)),K(this,t,null,rt,i,{})}}({target:document.body});
