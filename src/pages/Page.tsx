import { IonButton, IonButtons, IonContent, IonHeader, IonMenuButton, IonPage, IonTitle, IonToolbar } from '@ionic/react';
import { useParams } from 'react-router';
import ExploreContainer from '../components/ExploreContainer';
import './Page.css';
import { IonicPasskeys } from 'ionic-passkeys';
import { useEffect } from 'react';

import axios from "axios";

const client = axios.create({
   baseURL: "http://localhost/passkeyapi" 
});

const Page: React.FC = () => {

  const username = "justincrosbie+10@gmail.com";
  const displayName = "Le Duderino3";

  function registerPasskey() {

    console.log('In Page:useEffect');

    try {

      const fetchPost = async () => {
        const data = {username, displayName};

        let response = await client.post('start_registration', data);

        console.log('start_registration result');

        console.log(response.data);

        console.log('start_registration result ok');

        if ( response.data.challenge ) {

          const regresult = await IonicPasskeys.passkeyRegister({ challenge: response.data.challenge, username });
          console.log('passkeyRegister result');
          console.log(regresult);
          console.log('passkeyRegister result ok');

          const str = JSON.stringify(regresult.credential);
          console.log('str ok', str);

          if ( regresult.credential ) {
            const completiondata = {
              "username": username,
              "credential": regresult.credential
            };

            let completionresponse = await client.post('complete_registration', completiondata);

            console.log('start_registration result');

            console.log(completionresponse.data);
    
            console.log('start_registration result ok');
          }
        }
      };
      fetchPost();     
    } catch (error) {
      console.error(error);
    }

  };

  function authenticatePasskey() {

      const data = {
        "username": username
      };

      try {

        const fetchPost = async () => {
          let response = await client.post('start_authentication', data);

          if ( response.data.challenge ) {

            const authresult = await IonicPasskeys.passkeyAuthenticate({ challenge: response.data.challenge });

            if ( authresult.credential ) {
              const completiondata = {
                "credential": authresult.credential
              };

              let completionresponse = await client.post('complete_authentication', completiondata);

              console.log('complete_authentication result');

              console.log(completionresponse.data);
      
              console.log('complete_authentication result ok');
            }
          }
        };
        fetchPost();     
      } catch (error) {
        console.error(error);
      }
  }
    

  // const checkPasskeyOk = () => {
  //   // Availability of `window.PublicKeyCredential` means WebAuthn is usable.  
  //   // `isUserVerifyingPlatformAuthenticatorAvailable` means the feature detection is usable.  
  //   // `​​isConditionalMediationAvailable` means the feature detection is usable.  
  //   if (window.PublicKeyCredential &&  
  //     PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable &&  
  //     PublicKeyCredential.​​isConditionalMediationAvailable) {  
  //   // Check if user verifying platform authenticator is available.  
  //   Promise.all([  
  //     PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable(),  
  //     PublicKeyCredential.​​isConditionalMediationAvailable(),  
  //   ]).then(results => {  
  //     if (results.every(r => r === true)) {  
  //       // Display "Create a new passkey" button  
  //     }  
  //   });  
  //   }  
  // }

  const { name } = useParams<{ name: string; }>();

  return (
    <IonPage>
      <IonHeader>
        <IonToolbar>
          <IonButtons slot="start">
            <IonMenuButton />
          </IonButtons>
          <IonTitle>{name}</IonTitle>
        </IonToolbar>
      </IonHeader>

      <IonContent fullscreen>
        <IonHeader collapse="condense">
          <IonToolbar>
            <IonTitle size="large">{name}</IonTitle>
          </IonToolbar>
        </IonHeader>
        <ExploreContainer name={name} />
        <IonButton 
            onClick={() => {
              registerPasskey()
            }}
        >Register Passkey</IonButton>
        <IonButton 
            onClick={() => {
              authenticatePasskey()
            }}
        >Login Passkey</IonButton>
      </IonContent>
    </IonPage>
  );
};

export default Page;
