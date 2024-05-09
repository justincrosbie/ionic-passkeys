"use strict";(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[576],{576:function(e,t,r){r.r(t),r.d(t,{scopeCss:function(){return P}});let s=e=>e.replace(/[.*+?^${}()|[\]\\]/g,"\\$&"),l=e=>{let t=[],r=0;return{content:(e=e.replace(/(\[[^\]]*\])/g,(e,s)=>{let l=`__ph-${r}__`;return t.push(s),r++,l})).replace(/(:nth-[-\w]+)(\([^)]+\))/g,(e,s,l)=>{let c=`__ph-${r}__`;return t.push(l),r++,s+c}),placeholders:t}},c=(e,t)=>t.replace(/__ph-(\d+)__/g,(t,r)=>e[+r]),o="-shadowcsshost",n="-shadowcssslotted",p="-shadowcsscontext",i=")(?:\\(((?:\\([^)(]*\\)|[^)(]*)+?)\\))?([^,{]*)",a=RegExp("("+o+i,"gim"),h=RegExp("("+p+i,"gim"),u=RegExp("("+n+i,"gim"),g=o+"-no-combinator",d=/-shadowcsshost-no-combinator([^\s]*)/,m=[/::shadow/g,/::content/g],$=/-shadowcsshost/gim,f=e=>RegExp(`((?<!(^@supports(.*)))|(?<={.*))(${e}\\b)`,"gim"),_=f("::slotted"),x=f(":host"),S=f(":host-context"),b=/\/\*\s*[\s\S]*?\*\//g,E=e=>e.replace(b,""),w=/\/\*\s*#\s*source(Mapping)?URL=[\s\S]+?\*\//g,W=e=>e.match(w)||[],k=/(\s*)([^;\{\}]+?)(\s*)((?:{%BLOCK%}?\s*;?)|(?:\s*;))/g,O=/([{}])/g,R=/(^.*?[^\\])??((:+)(.*)|$)/,j="%BLOCK%",C=(e,t)=>{let r=L(e),s=0;return r.escapedString.replace(k,(...e)=>{let l=e[2],c="",o=e[4],n="";o&&o.startsWith("{"+j)&&(c=r.blocks[s++],o=o.substring(j.length+1),n="{");let p=t({selector:l,content:c});return`${e[1]}${p.selector}${e[3]}${n}${p.content}${o}`})},L=e=>{let t=e.split(O),r=[],s=[],l=0,c=[];for(let e=0;e<t.length;e++){let o=t[e];"}"===o&&l--,l>0?c.push(o):(c.length>0&&(s.push(c.join("")),r.push(j),c=[]),r.push(o)),"{"===o&&l++}return c.length>0&&(s.push(c.join("")),r.push(j)),{escapedString:r.join(""),blocks:s}},T=e=>e=e.replace(S,`$1${p}`).replace(x,`$1${o}`).replace(_,`$1${n}`),B=(e,t,r)=>e.replace(t,(...e)=>{if(!e[2])return g+e[3];{let t=e[2].split(","),s=[];for(let l=0;l<t.length;l++){let c=t[l].trim();if(!c)break;s.push(r(g,c,e[3]))}return s.join(",")}}),I=(e,t,r)=>e+t.replace(o,"")+r,K=e=>B(e,a,I),N=(e,t,r)=>t.indexOf(o)>-1?I(e,t,r):e+t+r+", "+t+" "+e+r,M=(e,t)=>{let r="."+t+" > ",s=[];return e=e.replace(u,(...e)=>{if(!e[2])return g+e[3];{let t=r+e[2].trim()+e[3],l="";for(let t=e[4]-1;t>=0;t--){let r=e[5][t];if("}"===r||","===r)break;l=r+l}let c=(l+t).trim(),o=`${l.trimEnd()}${t.trim()}`.trim();if(c!==o){let e=`${o}, ${c}`;s.push({orgSelector:c,updatedSelector:e})}return t}}),{selectors:s,cssText:e}},U=e=>B(e,h,N),q=e=>m.reduce((e,t)=>e.replace(t," "),e),v=e=>RegExp("^("+(e=e.replace(/\[/g,"\\[").replace(/\]/g,"\\]"))+")([>\\s~+[.,{:][\\s\\S]*)?$","m"),y=(e,t)=>!v(t).test(e),z=(e,t)=>e.replace(R,(e,r="",s,l="",c="")=>r+t+l+c),A=(e,t,r)=>{if($.lastIndex=0,$.test(e)){let t=`.${r}`;return e.replace(d,(e,r)=>z(r,t)).replace($,t+" ")}return t+" "+e},D=(e,t,r)=>{let s;let o="."+(t=t.replace(/\[is=([^\]]*)\]/g,(e,...t)=>t[0])),n=e=>{let s=e.trim();if(!s)return"";if(e.indexOf(g)>-1)s=A(e,t,r);else{let t=e.replace($,"");t.length>0&&(s=z(t,o))}return s},p=l(e);e=p.content;let i="",a=0,h=/( |>|\+|~(?!=))\s*/g,u=!(e.indexOf(g)>-1);for(;null!==(s=h.exec(e));){let t=s[1],r=e.slice(a,s.index).trim(),l=(u=u||r.indexOf(g)>-1)?n(r):r;i+=`${l} ${t} `,a=h.lastIndex}let d=e.substring(a);return i+=(u=u||d.indexOf(g)>-1)?n(d):d,c(p.placeholders,i)},F=(e,t,r,s)=>e.split(",").map(e=>s&&e.indexOf("."+s)>-1?e.trim():y(e,t)?D(e,t,r).trim():e.trim()).join(", "),G=(e,t,r,s,l)=>C(e,e=>{let l=e.selector,c=e.content;return"@"!==e.selector[0]?l=F(e.selector,t,r,s):(e.selector.startsWith("@media")||e.selector.startsWith("@supports")||e.selector.startsWith("@page")||e.selector.startsWith("@document"))&&(c=G(e.content,t,r,s)),{selector:l.replace(/\s{2,}/g," ").trim(),content:c}}),H=(e,t,r,s,l)=>{let c=M(e=U(e=K(e=T(e))),s);return e=q(e=c.cssText),t&&(e=G(e,t,r,s)),{cssText:(e=(e=J(e,r)).replace(/>\s*\*\s+([^{, ]+)/gm," $1 ")).trim(),slottedSelectors:c.selectors.map(e=>({orgSelector:J(e.orgSelector,r),updatedSelector:J(e.updatedSelector,r)}))}},J=(e,t)=>e.replace(/-shadowcsshost-no-combinator/g,`.${t}`),P=(e,t,r)=>{let l=W(e);e=E(e);let c=[];if(r){let t=e=>{let t=`/*!@___${c.length}___*/`,r=`/*!@${e.selector}*/`;return c.push({placeholder:t,comment:r}),e.selector=t+e.selector,e};e=C(e,e=>"@"!==e.selector[0]?t(e):((e.selector.startsWith("@media")||e.selector.startsWith("@supports")||e.selector.startsWith("@page")||e.selector.startsWith("@document"))&&(e.content=C(e.content,t)),e))}let o=H(e,t,t+"-h",t+"-s");return e=[o.cssText,...l].join("\n"),r&&c.forEach(({placeholder:t,comment:r})=>{e=e.replace(t,r)}),o.slottedSelectors.forEach(t=>{let r=RegExp(s(t.orgSelector),"g");e=e.replace(r,t.updatedSelector)}),e}}}]);