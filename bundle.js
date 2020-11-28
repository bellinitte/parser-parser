function t(){}function e(t){return t()}function n(){return Object.create(null)}function o(t){t.forEach(e)}function r(t){return"function"==typeof t}function i(t,e){return t!=t?e==e:t!==e||t&&"object"==typeof t||"function"==typeof t}function l(t,e){t.appendChild(e)}function c(t,e,n){t.insertBefore(e,n||null)}function s(t){t.parentNode.removeChild(t)}function u(t){return document.createElement(t)}function a(t){return document.createTextNode(t)}function d(){return a(" ")}function f(){return a("")}function m(t,e,n,o){return t.addEventListener(e,n,o),()=>t.removeEventListener(e,n,o)}function p(t,e,n){null==n?t.removeAttribute(e):t.getAttribute(e)!==n&&t.setAttribute(e,n)}function h(t,e){e=""+e,t.data!==e&&(t.data=e)}function g(t,e){for(let n=0;n<t.options.length;n+=1){const o=t.options[n];if(o.__value===e)return void(o.selected=!0)}}let b,v;function y(){if(void 0===b){b=!1;try{"undefined"!=typeof window&&window.parent&&window.parent.document}catch(t){b=!0}}return b}function $(t,e,n){t.classList[n?"add":"remove"](e)}function x(t){v=t}function w(){if(!v)throw new Error("Function called outside component initialization");return v}function k(){const t=w();return(e,n)=>{const o=t.$$.callbacks[e];if(o){const r=function(t,e){const n=document.createEvent("CustomEvent");return n.initCustomEvent(t,!1,!1,e),n}(e,n);o.slice().forEach(e=>{e.call(t,r)})}}}const _=[],C=[],E=[],N=[],A=Promise.resolve();let H=!1;function I(t){E.push(t)}let T=!1;const M=new Set;function z(){if(!T){T=!0;do{for(let t=0;t<_.length;t+=1){const e=_[t];x(e),S(e.$$)}for(_.length=0;C.length;)C.pop()();for(let t=0;t<E.length;t+=1){const e=E[t];M.has(e)||(M.add(e),e())}E.length=0}while(_.length);for(;N.length;)N.pop()();H=!1,T=!1,M.clear()}}function S(t){if(null!==t.fragment){t.update(),o(t.before_update);const e=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,e),t.after_update.forEach(I)}}const j=new Set;let B;function L(t,e){t&&t.i&&(j.delete(t),t.i(e))}function P(t,e,n,o){if(t&&t.o){if(j.has(t))return;j.add(t),B.c.push(()=>{j.delete(t),o&&(n&&t.d(1),o())}),t.o(e)}}function W(t,e){const n=e.token={};function r(t,r,i,l){if(e.token!==n)return;e.resolved=l;let c=e.ctx;void 0!==i&&(c=c.slice(),c[i]=l);const s=t&&(e.current=t)(c);let u=!1;e.block&&(e.blocks?e.blocks.forEach((t,n)=>{n!==r&&t&&(B={r:0,c:[],p:B},P(t,1,1,()=>{e.blocks[n]=null}),B.r||o(B.c),B=B.p)}):e.block.d(1),s.c(),L(s,1),s.m(e.mount(),e.anchor),u=!0),e.block=s,e.blocks&&(e.blocks[r]=s),u&&z()}if((i=t)&&"object"==typeof i&&"function"==typeof i.then){const n=w();if(t.then(t=>{x(n),r(e.then,1,e.value,t),x(null)},t=>{x(n),r(e.catch,2,e.error,t),x(null)}),e.current!==e.pending)return r(e.pending,0),!0}else{if(e.current!==e.then)return r(e.then,1,e.value,t),!0;e.resolved=t}var i}function O(t){t&&t.c()}function V(t,n,i){const{fragment:l,on_mount:c,on_destroy:s,after_update:u}=t.$$;l&&l.m(n,i),I(()=>{const n=c.map(e).filter(r);s?s.push(...n):o(n),t.$$.on_mount=[]}),u.forEach(I)}function K(t,e){const n=t.$$;null!==n.fragment&&(o(n.on_destroy),n.fragment&&n.fragment.d(e),n.on_destroy=n.fragment=null,n.ctx=[])}function q(t,e){-1===t.$$.dirty[0]&&(_.push(t),H||(H=!0,A.then(z)),t.$$.dirty.fill(0)),t.$$.dirty[e/31|0]|=1<<e%31}function J(e,r,i,l,c,u,a=[-1]){const d=v;x(e);const f=r.props||{},m=e.$$={fragment:null,ctx:null,props:u,update:t,not_equal:c,bound:n(),on_mount:[],on_destroy:[],before_update:[],after_update:[],context:new Map(d?d.$$.context:[]),callbacks:n(),dirty:a};let p=!1;if(m.ctx=i?i(e,f,(t,n,...o)=>{const r=o.length?o[0]:n;return m.ctx&&c(m.ctx[t],m.ctx[t]=r)&&(m.bound[t]&&m.bound[t](r),p&&q(e,t)),n}):[],m.update(),p=!0,o(m.before_update),m.fragment=!!l&&l(m.ctx),r.target){if(r.hydrate){const t=function(t){return Array.from(t.childNodes)}(r.target);m.fragment&&m.fragment.l(t),t.forEach(s)}else m.fragment&&m.fragment.c();r.intro&&L(e.$$.fragment),V(e,r.target,r.anchor),z()}x(d)}class F{$destroy(){K(this,1),this.$destroy=t}$on(t,e){const n=this.$$.callbacks[t]||(this.$$.callbacks[t]=[]);return n.push(e),()=>{const t=n.indexOf(e);-1!==t&&n.splice(t,1)}}$set(){}}function R(e){let n,o,r;return{c(){n=u("div"),o=u("textarea"),o.readOnly=!0,p(o,"class","svelte-1evpbm0"),p(n,"class","codemirror-container svelte-1evpbm0"),I(()=>e[18].call(n)),$(n,"flex",e[0])},m(t,i){c(t,n,i),l(n,o),e[17](o),r=function(t,e){const n=getComputedStyle(t),o=(parseInt(n.zIndex)||0)-1;"static"===n.position&&(t.style.position="relative");const r=u("iframe");r.setAttribute("style",`display: block; position: absolute; top: 0; left: 0; width: 100%; height: 100%; overflow: hidden; border: 0; opacity: 0; pointer-events: none; z-index: ${o};`),r.setAttribute("aria-hidden","true"),r.tabIndex=-1;const i=y();let c;return i?(r.src="data:text/html,<script>onresize=function(){parent.postMessage(0,'*')}<\/script>",c=m(window,"message",t=>{t.source===r.contentWindow&&e()})):(r.src="about:blank",r.onload=()=>{c=m(r.contentWindow,"resize",e)}),l(t,r),()=>{(i||c&&r.contentWindow)&&c(),s(r)}}(n,e[18].bind(n))},p(t,[e]){1&e&&$(n,"flex",t[0])},i:t,o:t,d(t){t&&s(n),e[17](null),r()}}}function U(t){return new Promise(e=>setTimeout(e,t))}function D(t,e,n){const o=k();let r,i,l,c,{readonly:s=!1}=e,{flex:u=!1}=e,{lineNumbers:a=!0}=e,{tab:d=!0}=e,{lint:f=null}=e,{mode:m="ebnf"}=e,p="";let h,g=!1,b=!1;var v;v=()=>((async()=>{let t=await import("./codemirror-9ea8e240.js");h=t.default,await async function(t){if(b||!h)return;c&&c.toTextArea();const e={lineNumbers:a,lineWrapping:!0,indentWithTabs:!0,indentUnit:2,tabSize:2,tabIndex:4,value:"",mode:{name:t},readOnly:s,autoCloseBrackets:!0,autoCloseTags:!0,extraKeys:{Enter:"newlineAndIndentContinueMarkdownList","Ctrl-/":"toggleComment","Cmd-/":"toggleComment"},lint:!!f&&{getAnnotations:f,delay:Number.EPSILON},theme:"gruvbox-dark"};d||(e.extraKeys.Tab=d,e.extraKeys["Shift-Tab"]=d),y&&await U(50),b||(n(20,c=h.fromTextArea(l,e)),c.on("change",t=>{if(!g){const e=t.getValue();o("change",{value:e})}}),y&&await U(50),c.refresh(),y=!1)}(m),c&&c.setValue(p||"")})(),()=>{b=!0,c&&c.toTextArea()}),w().$$.on_mount.push(v);let y=!0;return t.$set=t=>{"readonly"in t&&n(4,s=t.readonly),"flex"in t&&n(0,u=t.flex),"lineNumbers"in t&&n(5,a=t.lineNumbers),"tab"in t&&n(6,d=t.tab),"lint"in t&&n(7,f=t.lint),"mode"in t&&n(8,m=t.mode)},t.$$.update=()=>{1048582&t.$$.dirty&&c&&r&&i&&c.refresh()},[u,r,i,l,s,a,d,f,m,async function(t){p=t,g=!0,c&&c.setValue(p),g=!1},function(){return c.getValue()},function(t){if(p=t,c){const{left:e,top:n}=c.getScrollInfo();c.setValue(p=t),c.scrollTo(e,n)}},function(){c.refresh()},function(){c.focus()},function(){return c.getHistory()},function(t){c.setHistory(t)},function(){c&&c.clearHistory()},function(t){C[t?"unshift":"push"](()=>{l=t,n(3,l)})},function(){r=this.offsetWidth,i=this.offsetHeight,n(1,r),n(2,i)}]}class G extends F{constructor(t){var e;super(),document.getElementById("svelte-1evpbm0-style")||((e=u("style")).id="svelte-1evpbm0-style",e.textContent='.codemirror-container.svelte-1evpbm0{position:relative;width:100%;height:100%;border:none;line-height:1.5;overflow:hidden}.codemirror-container.svelte-1evpbm0 .CodeMirror{height:100%;background:transparent;font:400 16px/1.7;font-family:"JetBrains Mono", Consolas, monospace;color:var(--base)}.codemirror-container.flex.svelte-1evpbm0 .CodeMirror{height:auto}.codemirror-container.flex.svelte-1evpbm0 .CodeMirror-lines{padding:0}.codemirror-container.svelte-1evpbm0 .CodeMirror-gutters{padding:0 16px 0 8px;border:none}.codemirror-container.svelte-1evpbm0 .error-loc{position:relative;border-bottom:2px solid #da106e}.codemirror-container.svelte-1evpbm0 .error-line{background-color:rgba(200, 0, 0, 0.05)}textarea.svelte-1evpbm0{visibility:hidden}',l(document.head,e)),J(this,t,D,R,i,{readonly:4,flex:0,lineNumbers:5,tab:6,lint:7,mode:8,set:9,get:10,update:11,resize:12,focus:13,getHistory:14,setHistory:15,clearHistory:16})}get set(){return this.$$.ctx[9]}get get(){return this.$$.ctx[10]}get update(){return this.$$.ctx[11]}get resize(){return this.$$.ctx[12]}get focus(){return this.$$.ctx[13]}get getHistory(){return this.$$.ctx[14]}get setHistory(){return this.$$.ctx[15]}get clearHistory(){return this.$$.ctx[16]}}function Q(t,e,n){const o=t.slice();return o[16]=e[n],o}function X(t){let e,n,o,r,i=t[16]+"";return{c(){e=u("option"),n=a(i),o=d(),e.__value=r=t[16],e.value=e.__value},m(t,r){c(t,e,r),l(e,n),l(e,o)},p(t,o){8&o&&i!==(i=t[16]+"")&&h(n,i),8&o&&r!==(r=t[16])&&(e.__value=r),e.value=e.__value},d(t){t&&s(e)}}}function Y(t){let e,n,o="> "+t[5];return{c(){e=u("p"),n=a(o),p(e,"class","output svelte-oy86wb")},m(t,o){c(t,e,o),l(e,n)},p(t,e){32&e&&o!==(o="> "+t[5])&&h(n,o)},d(t){t&&s(e)}}}function Z(t){let e,n,r,i,a,f,h,b,v,y,$,x,w,k={lint:t[9]};r=new G({props:k}),t[11](r),r.$on("change",t[6]);let _=t[3],C=[];for(let e=0;e<_.length;e+=1)C[e]=X(Q(t,_,e));v=new G({props:{mode:"text"}}),t[13](v),v.$on("change",t[7]);let E=""!=t[5]&&Y(t);return{c(){e=u("div"),n=u("div"),O(r.$$.fragment),i=d(),a=u("select");for(let t=0;t<C.length;t+=1)C[t].c();h=d(),b=u("div"),O(v.$$.fragment),y=d(),E&&E.c(),p(n,"class","editor-left svelte-oy86wb"),a.disabled=f=!t[2],void 0===t[4]&&I(()=>t[12].call(a)),p(b,"class","editor-right svelte-oy86wb"),p(e,"class","container svelte-oy86wb")},m(o,s){c(o,e,s),l(e,n),V(r,n,null),l(e,i),l(e,a);for(let t=0;t<C.length;t+=1)C[t].m(a,null);g(a,t[4]),l(e,h),l(e,b),V(v,b,null),l(e,y),E&&E.m(e,null),$=!0,x||(w=[m(a,"change",t[12]),m(a,"change",t[8])],x=!0)},p(t,[n]){if(r.$set({}),8&n){let e;for(_=t[3],e=0;e<_.length;e+=1){const o=Q(t,_,e);C[e]?C[e].p(o,n):(C[e]=X(o),C[e].c(),C[e].m(a,null))}for(;e<C.length;e+=1)C[e].d(1);C.length=_.length}(!$||4&n&&f!==(f=!t[2]))&&(a.disabled=f),24&n&&g(a,t[4]);v.$set({}),""!=t[5]?E?E.p(t,n):(E=Y(t),E.c(),E.m(e,null)):E&&(E.d(1),E=null)},i(t){$||(L(r.$$.fragment,t),L(v.$$.fragment,t),$=!0)},o(t){P(r.$$.fragment,t),P(v.$$.fragment,t),$=!1},d(n){n&&s(e),t[11](null),K(r),function(t,e){for(let n=0;n<t.length;n+=1)t[n]&&t[n].d(e)}(C,n),t[13](null),K(v),E&&E.d(),x=!1,o(w)}}}function tt(t,e,n){let o,r,i,l,c,{core:s}=e,u=[],a="";function d(t){i&&n(5,a=i.check(t,l)?"success":"failure")}return t.$set=t=>{"core"in t&&n(10,s=t.core)},[o,r,i,u,l,a,async function(t){n(2,i=void 0),n(5,a="");try{n(2,i=new s.EbnfParserParser(t.detail.value)),n(3,u=i.productionRules),null!==l&&u.includes(l)||n(4,l=u[0]),c=null,d(r.get())}catch(t){console.error(t),c={message:t.kind,from:{line:t.span.from.line,ch:t.span.from.ch},to:{line:t.span.to.line,ch:t.span.to.ch}}}},function(t){d(t.detail.value)},function(t){d(r.get())},function(){let t=[];return c&&t.push(c),t},s,function(t){C[t?"unshift":"push"](()=>{o=t,n(0,o)})},function(){l=function(t){const e=t.querySelector(":checked")||t.options[0];return e&&e.__value}(this),n(4,l),n(3,u)},function(t){C[t?"unshift":"push"](()=>{r=t,n(1,r)})}]}class et extends F{constructor(t){var e;super(),document.getElementById("svelte-oy86wb-style")||((e=u("style")).id="svelte-oy86wb-style",e.textContent='.container.svelte-oy86wb{position:relative;width:100%;height:100%;display:inline-block}.editor-left.svelte-oy86wb{width:50%;height:100%;float:left}.editor-right.svelte-oy86wb{width:50%;height:15%;float:left}.output.svelte-oy86wb{float:left;font-family:"JetBrains Mono", Consolas, monospace;color:#928374;padding-left:24px}',l(document.head,e)),J(this,t,tt,Z,i,{core:10})}}function nt(e){let n,o,r=e[2].message+"";return{c(){var t,e,i;n=u("p"),o=a(r),t="color",e="red",n.style.setProperty(t,e,i?"important":"")},m(t,e){c(t,n,e),l(n,o)},i:t,o:t,d(t){t&&s(n)}}}function ot(t){let e,n,o,r={ctx:t,current:null,token:null,pending:lt,then:it,catch:rt,value:1,blocks:[,,,]};return W(n=t[0].default(),r),{c(){e=f(),r.block.c()},m(t,n){c(t,e,n),r.block.m(t,r.anchor=n),r.mount=()=>e.parentNode,r.anchor=e,o=!0},p(e,n){t=e},i(t){o||(L(r.block),o=!0)},o(t){for(let t=0;t<3;t+=1){P(r.blocks[t])}o=!1},d(t){t&&s(e),r.block.d(t),r.token=null,r=null}}}function rt(e){return{c:t,m:t,i:t,o:t,d:t}}function it(t){let e,n;return e=new et({props:{core:t[1]}}),{c(){O(e.$$.fragment)},m(t,o){V(e,t,o),n=!0},i(t){n||(L(e.$$.fragment,t),n=!0)},o(t){P(e.$$.fragment,t),n=!1},d(t){K(e,t)}}}function lt(e){let n;return{c(){n=u("p"),n.textContent="Loading core..."},m(t,e){c(t,n,e)},i:t,o:t,d(t){t&&s(n)}}}function ct(e){let n;return{c(){n=u("p"),n.textContent="Loading module..."},m(t,e){c(t,n,e)},i:t,o:t,d(t){t&&s(n)}}}function st(t){let e,n,o,r={ctx:t,current:null,token:null,pending:ct,then:ot,catch:nt,value:0,error:2,blocks:[,,,]};return W(n=import("./Cargo-c529ac2e.js"),r),{c(){e=f(),r.block.c()},m(t,n){c(t,e,n),r.block.m(t,r.anchor=n),r.mount=()=>e.parentNode,r.anchor=e,o=!0},p(e,[n]){t=e},i(t){o||(L(r.block),o=!0)},o(t){for(let t=0;t<3;t+=1){P(r.blocks[t])}o=!1},d(t){t&&s(e),r.block.d(t),r.token=null,r=null}}}new class extends F{constructor(t){var e;super(),document.getElementById("svelte-1vl3ij0-style")||((e=u("style")).id="svelte-1vl3ij0-style",e.textContent="body{background-color:rgb(40, 40, 40)}",l(document.head,e)),J(this,t,null,st,i,{})}}({target:document.body});
