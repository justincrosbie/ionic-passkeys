"use strict";(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[789],{7789:function(t,e,r){let s;r.r(e),r.d(e,{createSwipeBackGesture:function(){return X}});var i=r(6587),l=r(545);/*!
 * (C) Ionic http://ionicframework.com - MIT License
 */class a{constructor(){this.gestureId=0,this.requestedStart=new Map,this.disabledGestures=new Map,this.disabledScroll=new Set}createGesture(t){var e;return new n(this,this.newID(),t.name,null!==(e=t.priority)&&void 0!==e?e:0,!!t.disableScroll)}createBlocker(t={}){return new d(this,this.newID(),t.disable,!!t.disableScroll)}start(t,e,r){return this.canStart(t)?(this.requestedStart.set(e,r),!0):(this.requestedStart.delete(e),!1)}capture(t,e,r){if(!this.start(t,e,r))return!1;let s=this.requestedStart,i=-1e4;if(s.forEach(t=>{i=Math.max(i,t)}),i===r){this.capturedId=e,s.clear();let r=new CustomEvent("ionGestureCaptured",{detail:{gestureName:t}});return document.dispatchEvent(r),!0}return s.delete(e),!1}release(t){this.requestedStart.delete(t),this.capturedId===t&&(this.capturedId=void 0)}disableGesture(t,e){let r=this.disabledGestures.get(t);void 0===r&&(r=new Set,this.disabledGestures.set(t,r)),r.add(e)}enableGesture(t,e){let r=this.disabledGestures.get(t);void 0!==r&&r.delete(e)}disableScroll(t){this.disabledScroll.add(t),1===this.disabledScroll.size&&document.body.classList.add(c)}enableScroll(t){this.disabledScroll.delete(t),0===this.disabledScroll.size&&document.body.classList.remove(c)}canStart(t){return!(void 0!==this.capturedId||this.isDisabled(t))}isCaptured(){return void 0!==this.capturedId}isScrollDisabled(){return this.disabledScroll.size>0}isDisabled(t){let e=this.disabledGestures.get(t);return!!e&&e.size>0}newID(){return this.gestureId++,this.gestureId}}class n{constructor(t,e,r,s,i){this.id=e,this.name=r,this.disableScroll=i,this.priority=1e6*s+e,this.ctrl=t}canStart(){return!!this.ctrl&&this.ctrl.canStart(this.name)}start(){return!!this.ctrl&&this.ctrl.start(this.name,this.id,this.priority)}capture(){if(!this.ctrl)return!1;let t=this.ctrl.capture(this.name,this.id,this.priority);return t&&this.disableScroll&&this.ctrl.disableScroll(this.id),t}release(){this.ctrl&&(this.ctrl.release(this.id),this.disableScroll&&this.ctrl.enableScroll(this.id))}destroy(){this.release(),this.ctrl=void 0}}class d{constructor(t,e,r,s){this.id=e,this.disable=r,this.disableScroll=s,this.ctrl=t}block(){if(this.ctrl){if(this.disable)for(let t of this.disable)this.ctrl.disableGesture(t,this.id);this.disableScroll&&this.ctrl.disableScroll(this.id)}}unblock(){if(this.ctrl){if(this.disable)for(let t of this.disable)this.ctrl.enableGesture(t,this.id);this.disableScroll&&this.ctrl.enableScroll(this.id)}}destroy(){this.unblock(),this.ctrl=void 0}}let c="backdrop-no-scroll",o=new a,u=(t,e,r,s)=>{let i,l;let a=h(t)?{capture:!!s.capture,passive:!!s.passive}:!!s.capture;return t.__zone_symbol__addEventListener?(i="__zone_symbol__addEventListener",l="__zone_symbol__removeEventListener"):(i="addEventListener",l="removeEventListener"),t[i](e,r,a),()=>{t[l](e,r,a)}},h=t=>{if(void 0===s)try{let e=Object.defineProperty({},"passive",{get:()=>{s=!0}});t.addEventListener("optsTest",()=>{},e)}catch(t){s=!1}return!!s},b=(t,e,r,s,i)=>{let l,a,n,d,c,o,h;let b=0,m=s=>{b=Date.now()+2e3,e(s)&&(!a&&r&&(a=u(t,"touchmove",r,i)),n||(n=u(s.target,"touchend",S,i)),d||(d=u(s.target,"touchcancel",S,i)))},p=s=>{!(b>Date.now())&&e(s)&&(!o&&r&&(o=u(v(t),"mousemove",r,i)),h||(h=u(v(t),"mouseup",y,i)))},S=t=>{f(),s&&s(t)},y=t=>{X(),s&&s(t)},f=()=>{a&&a(),n&&n(),d&&d(),a=n=d=void 0},X=()=>{o&&o(),h&&h(),o=h=void 0},g=()=>{f(),X()},w=(e=!0)=>{e?(l||(l=u(t,"touchstart",m,i)),c||(c=u(t,"mousedown",p,i))):(l&&l(),c&&c(),l=c=void 0,g())};return{enable:w,stop:g,destroy:()=>{w(!1),s=r=e=void 0}}},v=t=>t instanceof Document?t:t.ownerDocument,m=(t,e,r)=>{let s="x"===t,i=Math.cos(Math.PI/180*r),l=e*e,a=0,n=0,d=!1,c=0;return{start(t,e){a=t,n=e,c=0,d=!0},detect(t,e){if(!d)return!1;let r=t-a,o=e-n,u=r*r+o*o;if(u<l)return!1;let h=(s?r:o)/Math.sqrt(u);return c=h>i?1:h<-i?-1:0,d=!1,!0},isGesture:()=>0!==c,getDirection:()=>c}},p=t=>{let e=!1,r=!1,s=!0,i=!1,l=Object.assign({disableScroll:!1,direction:"x",gesturePriority:0,passive:!0,maxAngle:40,threshold:10},t),a=l.canStart,n=l.onWillStart,d=l.onStart,c=l.onEnd,u=l.notCaptured,h=l.onMove,v=l.threshold,p=l.passive,X=l.blurOnStart,g={type:"pan",startX:0,startY:0,startTime:0,currentX:0,currentY:0,velocityX:0,velocityY:0,deltaX:0,deltaY:0,currentTime:0,event:void 0,data:void 0},w=m(l.direction,l.threshold,l.maxAngle),Y=o.createGesture({name:t.gestureName,priority:t.gesturePriority,disableScroll:t.disableScroll}),_=()=>{e&&(i=!1,h&&h(g))},G=()=>!!Y.capture()&&(e=!0,s=!1,g.startX=g.currentX,g.startY=g.currentY,g.startTime=g.currentTime,n?n(g).then(D):D(),!0),E=()=>{if("undefined"!=typeof document){let t=document.activeElement;(null==t?void 0:t.blur)&&t.blur()}},D=()=>{X&&E(),d&&d(g),s=!0},I=()=>{e=!1,r=!1,i=!1,s=!0,Y.release()},k=t=>{let r=e,i=s;if(I(),i){if(S(g,t),r){c&&c(g);return}u&&u(g)}},M=b(l.el,t=>{let e=f(t);return!r&&!!s&&(y(t,g),g.startX=g.currentX,g.startY=g.currentY,g.startTime=g.currentTime=e,g.velocityX=g.velocityY=g.deltaX=g.deltaY=0,g.event=t,(!a||!1!==a(g))&&(Y.release(),!!Y.start()&&((r=!0,0===v)?G():(w.start(g.startX,g.startY),!0))))},t=>{if(e){!i&&s&&(i=!0,S(g,t),requestAnimationFrame(_));return}S(g,t),!w.detect(g.currentX,g.currentY)||w.isGesture()&&G()||T()},k,{capture:!1,passive:p}),T=()=>{I(),M.stop(),u&&u(g)};return{enable(t=!0){t||(e&&k(void 0),I()),M.enable(t)},destroy(){Y.destroy(),M.destroy()}}},S=(t,e)=>{if(!e)return;let r=t.currentX,s=t.currentY,i=t.currentTime;y(e,t);let l=t.currentX,a=t.currentY,n=(t.currentTime=f(e))-i;n>0&&n<100&&(t.velocityX=(l-r)/n*.7+.3*t.velocityX,t.velocityY=(a-s)/n*.7+.3*t.velocityY),t.deltaX=l-t.startX,t.deltaY=a-t.startY,t.event=e},y=(t,e)=>{let r=0,s=0;if(t){let e=t.changedTouches;if(e&&e.length>0){let t=e[0];r=t.clientX,s=t.clientY}else void 0!==t.pageX&&(r=t.pageX,s=t.pageY)}e.currentX=r,e.currentY=s},f=t=>t.timeStamp||Date.now(),X=(t,e,r,s,a)=>{let n=t.ownerDocument.defaultView,d=(0,l.i)(t),c=t=>{let{startX:e}=t;return d?e>=n.innerWidth-50:e<=50},o=t=>d?-t.deltaX:t.deltaX,u=t=>d?-t.velocityX:t.velocityX;return p({el:t,gestureName:"goback-swipe",gesturePriority:101,threshold:10,canStart:r=>(d=(0,l.i)(t),c(r)&&e()),onStart:r,onMove:t=>{s(o(t)/n.innerWidth)},onEnd:t=>{let e=o(t),r=n.innerWidth,s=e/r,l=u(t),d=l>=0&&(l>.2||e>r/2),c=(d?1-s:s)*r,h=0;c>5&&(h=Math.min(c/Math.abs(l),540)),a(d,s<=0?.01:(0,i.m)(0,s,.9999),h)}})}}}]);