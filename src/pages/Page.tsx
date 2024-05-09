import { IonButton, IonButtons, IonContent, IonHeader, IonMenuButton, IonPage, IonTitle, IonToolbar } from '@ionic/react';
import { useParams } from 'react-router';
import ExploreContainer from '../components/ExploreContainer';
import { IonicPasskeys } from 'ionic-passkeys';
import { useEffect } from 'react';

import axios from "axios";

const client = axios.create({
   baseURL: "https://0d4b-58-104-243-37.ngrok-free.app/api" 
});

const Page: React.FC = () => {

  const username = "justincrosbie+15@gmail.com";
  const displayName = "El Duderino";

  function registerPasskey() {

    console.log('In Page:useEffect: ' + "https://0d4b-58-104-243-37.ngrok-free.app/api");

      const fetchPost = async () => {
        try {

          const data = {username, displayName};

            console.log('Doing start_registration');

            let response = await client.post('start_registration', data);

            console.log('start_registration result');

            console.log(response.data);

            console.log('start_registration result ok');

            if ( response.data.challenge ) {

              const regresult = await IonicPasskeys.passkeyRegister({ challenge: JSON.stringify(response.data.challenge) });
              console.log('passkeyRegister result');
              console.log(regresult);
              console.log('passkeyRegister credential');
              console.log(regresult.credential);
              console.log('passkeyRegister result ok');

              if ( !regresult.credential ) {
                console.log('no credential');
                return;
              }

              const str = regresult.credential;
              console.log('str ok', str);

              if ( regresult.credential ) {
                const completiondata = {
                  "username": username,
                  "credential": JSON.parse(regresult.credential)
                };

                console.log('doing complete_registration');

                try {
                  let completionresponse = await client.post('complete_registration', completiondata);

                  console.log('complete_registration result');
      
                  console.log(completionresponse.data);
          
                  console.log('complete_registration result ok');
                  } catch (error) {
                    console.error('complete_registration error');
                    console.error(error);
                  }
              }
            }
          } catch (error) {
            console.error(error);
          }
        };
      fetchPost();     

  };

  function authenticatePasskey() {

      const data = {
        "username": username
      };

        const fetchPost = async () => {

          try {

            console.log('Doing start_authentication');

            let response = await client.post('start_authentication', data);

            console.log('start_authentication result');

            console.log(response.data);
    
            console.log('start_authentication result ok');
              
            if ( response.data.challenge ) {

              const authresult = await IonicPasskeys.passkeyAuthenticate({ challenge: JSON.stringify(response.data.challenge) });

              console.log('passkeyRegister result');
              console.log(authresult);
              console.log('passkeyRegister credential');
              console.log(authresult.credential);
              console.log('passkeyRegister result ok');

              if ( !authresult.credential ) {
                console.log('no credential');
                return;
              }

              const str = authresult.credential;
              console.log('str ok', str);

              if ( authresult.credential ) {
                const completiondata = {
                  "credential": JSON.parse(authresult.credential)
                };

                console.log('doing complete_authentication');

                let completionresponse = await client.post('complete_authentication', completiondata);

                console.log('complete_authentication result');

                console.log(completionresponse.data);
        
                console.log('complete_authentication result ok');
              }
            }
          } catch (error) {
            console.error(error);
          }
        };
        fetchPost();     
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
