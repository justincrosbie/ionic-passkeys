"use strict";(()=>{var e={};e.id=535,e.ids=[535,660],e.modules={323:(e,t)=>{Object.defineProperty(t,"l",{enumerable:!0,get:function(){return function e(t,r){return r in t?t[r]:"then"in t&&"function"==typeof t.then?t.then(t=>e(t,r)):"function"==typeof t&&"default"===r?t:void 0}}})},100:(e,t,r)=>{r.a(e,async(e,a)=>{try{r.r(t),r.d(t,{config:()=>y,default:()=>p,getServerSideProps:()=>f,getStaticPaths:()=>b,getStaticProps:()=>h,reportWebVitals:()=>P,routeModule:()=>S,unstable_getServerProps:()=>_,unstable_getServerSideProps:()=>x,unstable_getStaticParams:()=>m,unstable_getStaticPaths:()=>O,unstable_getStaticProps:()=>j});var s=r(93),n=r(244),o=r(323),i=r(777),l=r.n(i),c=r(863),u=r.n(c),d=r(514),g=e([d]);d=(g.then?(await g)():g)[0];let p=(0,o.l)(d,"default"),h=(0,o.l)(d,"getStaticProps"),b=(0,o.l)(d,"getStaticPaths"),f=(0,o.l)(d,"getServerSideProps"),y=(0,o.l)(d,"config"),P=(0,o.l)(d,"reportWebVitals"),j=(0,o.l)(d,"unstable_getStaticProps"),O=(0,o.l)(d,"unstable_getStaticPaths"),m=(0,o.l)(d,"unstable_getStaticParams"),_=(0,o.l)(d,"unstable_getServerProps"),x=(0,o.l)(d,"unstable_getServerSideProps"),S=new s.PagesRouteModule({definition:{kind:n.x.PAGES,page:"/Page",pathname:"/Page",bundlePath:"",filename:""},components:{App:u(),Document:l()},userland:d});a()}catch(e){a(e)}})},395:(e,t,r)=>{var a=r(444);let s=a.registerPlugin("IonicPasskeys",{web:()=>Promise.resolve().then(function(){return o}).then(e=>new e.IonicPasskeysWeb)});class n extends a.WebPlugin{async passkeyRegister(e){try{let t=JSON.parse(e.challenge),r={publicKey:Object.assign(Object.assign({},t.publicKey),{challenge:this.base64ToArrayBuffer(t.publicKey.challenge),user:Object.assign(Object.assign({},t.publicKey.user),{id:this.base64ToArrayBuffer(t.publicKey.user.id)}),excludeCredentials:t.publicKey.excludeCredentials.map(e=>Object.assign(Object.assign({},e),{id:this.base64ToArrayBuffer(e.id)}))})};console.log("publicKeyCredentialCreationOptions",r);let a=await navigator.credentials.create(r),s=a||{},n=Object.assign(Object.assign({},s),{id:s.id,type:s.type,rawId:this.base64UrlEncode(s.rawId),response:Object.assign(Object.assign({},s.response),{clientDataJSON:this.base64UrlEncode(s.response.clientDataJSON),attestationObject:this.base64UrlEncode(s.response.attestationObject)})});if(!a)throw Error("Failed to create credentials");return{credential:JSON.stringify(n)}}catch(e){throw Error(`Registration failed: ${e.message||"An unknown error occurred"}`)}}async passkeyAuthenticate(e){try{let t=JSON.parse(e.challenge),r={publicKey:Object.assign(Object.assign({},t.publicKey),{challenge:this.base64ToArrayBuffer(t.publicKey.challenge),allowCredentials:t.publicKey.allowCredentials.map(e=>Object.assign(Object.assign({},e),{id:this.base64ToArrayBuffer(e.id)}))})},a=await navigator.credentials.get(r),s=a||{},n=Object.assign(Object.assign({},s),{authenticatorAttachment:s.authenticatorAttachment,id:s.id,type:s.type,rawId:this.base64UrlEncode(s.rawId),response:Object.assign(Object.assign({},s.response),{clientDataJSON:this.base64UrlEncode(s.response.clientDataJSON),authenticatorData:this.base64UrlEncode(s.response.authenticatorData),signature:this.base64UrlEncode(s.response.signature),userHandle:this.base64UrlEncode(s.response.userHandle)})});if(!a)throw Error("Failed to create credentials");return{credential:JSON.stringify(n)}}catch(e){throw Error(`Authentication failed: ${e.message||"An unknown error occurred"}`)}}base64UrlEncode(e){let t=new Uint8Array(e),r="";for(let e=0;e<t.length;e++)r+=String.fromCharCode(t[e]);return btoa(r).replace(/\+/g,"-").replace(/\//g,"_").replace(/=+$/,"")}base64ToArrayBuffer(e){let t=atob(e=e.replace(/-/g,"+").replace(/_/g,"/")),r=new Uint8Array(t.length);for(let e=0;e<t.length;e++)r[e]=t.charCodeAt(e);return r.buffer}}var o=Object.freeze({__proto__:null,IonicPasskeysWeb:n});t.r=s},863:(e,t,r)=>{Object.defineProperty(t,"__esModule",{value:!0}),Object.defineProperty(t,"default",{enumerable:!0,get:function(){return l}});let a=r(167),s=r(997),n=a._(r(689)),o=r(921);async function i(e){let{Component:t,ctx:r}=e;return{pageProps:await (0,o.loadGetInitialProps)(t,r)}}class l extends n.default.Component{render(){let{Component:e,pageProps:t}=this.props;return(0,s.jsx)(e,{...t})}}l.origGetInitialProps=i,l.getInitialProps=i,("function"==typeof t.default||"object"==typeof t.default&&null!==t.default)&&void 0===t.default.__esModule&&(Object.defineProperty(t.default,"__esModule",{value:!0}),Object.assign(t.default,t),e.exports=t.default)},823:(e,t,r)=>{r.d(t,{Z:()=>s});var a=r(997);let s=({name:e})=>(0,a.jsxs)("div",{id:"container",children:[a.jsx("strong",{children:e}),(0,a.jsxs)("p",{children:["Explore ",a.jsx("a",{target:"_blank",rel:"noopener noreferrer",href:"https://ionicframework.com/docs/components",children:"UI Components"})]})]})},514:(e,t,r)=>{r.a(e,async(e,a)=>{try{r.r(t),r.d(t,{default:()=>g});var s=r(997),n=r(664),o=r(308),i=r(823),l=r(395),c=r(648),u=e([c]);let d=(c=(u.then?(await u)():u)[0]).default.create({baseURL:"http://localhost/passkeyapi"}),g=()=>{let e="justincrosbie+11@gmail.com",{name:t}=(0,o.useParams)();return(0,s.jsxs)(n.IonPage,{children:[s.jsx(n.IonHeader,{children:(0,s.jsxs)(n.IonToolbar,{children:[s.jsx(n.IonButtons,{slot:"start",children:s.jsx(n.IonMenuButton,{})}),s.jsx(n.IonTitle,{children:t})]})}),(0,s.jsxs)(n.IonContent,{fullscreen:!0,children:[s.jsx(n.IonHeader,{collapse:"condense",children:s.jsx(n.IonToolbar,{children:s.jsx(n.IonTitle,{size:"large",children:t})})}),s.jsx(i.Z,{name:t}),s.jsx(n.IonButton,{onClick:()=>{!function(){console.log("In Page:useEffect");try{(async()=>{let t=await d.post("start_registration",{username:e,displayName:"El Duderino4"});if(console.log("start_registration result"),console.log(t.data),console.log("start_registration result ok"),t.data.challenge){let r=await l.r.passkeyRegister({challenge:JSON.stringify(t.data.challenge)});console.log("passkeyRegister result"),console.log(r),console.log("passkeyRegister result ok");let a=r.credential;if(console.log("str ok",a),r.credential){let t={username:e,credential:JSON.parse(r.credential)},a=await d.post("complete_registration",t);console.log("start_registration result"),console.log(a.data),console.log("start_registration result ok")}}})()}catch(e){console.error(e)}}()},children:"Register Passkey"}),s.jsx(n.IonButton,{onClick:()=>{!function(){let t={username:e};try{(async()=>{let e=await d.post("start_authentication",t);if(e.data.challenge){let t=await l.r.passkeyAuthenticate({challenge:JSON.stringify(e.data.challenge)});if(t.credential){let e={credential:JSON.parse(t.credential)},r=await d.post("complete_authentication",e);console.log("complete_authentication result"),console.log(r.data),console.log("complete_authentication result ok")}}})()}catch(e){console.error(e)}}()},children:"Login Passkey"})]})]})};a()}catch(e){a(e)}})},244:(e,t)=>{var r;Object.defineProperty(t,"x",{enumerable:!0,get:function(){return r}}),function(e){e.PAGES="PAGES",e.PAGES_API="PAGES_API",e.APP_PAGE="APP_PAGE",e.APP_ROUTE="APP_ROUTE"}(r||(r={}))},444:e=>{e.exports=require("@capacitor/core")},664:e=>{e.exports=require("@ionic/react")},785:e=>{e.exports=require("next/dist/compiled/next-server/pages.runtime.prod.js")},689:e=>{e.exports=require("react")},308:e=>{e.exports=require("react-router")},997:e=>{e.exports=require("react/jsx-runtime")},648:e=>{e.exports=import("axios")},17:e=>{e.exports=require("path")}};var t=require("../webpack-runtime.js");t.C(e);var r=e=>t(t.s=e),a=t.X(0,[777],()=>r(100));module.exports=a})();