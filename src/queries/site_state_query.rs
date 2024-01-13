use lemmy_api_common::site::GetSiteResponse;
use leptos::*;
use leptos_query::{use_query, QueryOptions, QueryResult, RefetchFn, ResourceOption};

use crate::errors::LemmyAppError;

#[server(GetSiteResource, "/serverfn", "GetJson")]
async fn get_site() -> Result<GetSiteResponse, ServerFnError> {
  // use crate::lemmy_client::LemmyClient;
  use crate::lemmy_client::*;
  use actix_session::Session;
  // use actix_web::web;
  use leptos_actix::extract;

  let jwt = extract(|session: Session| async move {
      session.get::<String>("jwt")
  })
  .await??;

  logging::log!("SITE JWT {:#?}", jwt);

  let result = (Fetch {}).get_site(jwt).await?;

  // logging::log!("coop {:#?}", result);

  Ok(result)



  // Ok(
  //   extract(
  //     |/* session: Session,  */client: web::Data<awc::Client>| async move {
  //       // let jwt = session.get::<String>("jwt")?;

  //       client.get_site(/* jwt */).await
  //     },
  //   )
  //   .await??,
  // )
}

pub fn use_site_state() -> QueryResult<Result<GetSiteResponse, LemmyAppError>, impl RefetchFn> {

  use_query(
    || (),
    |_| async move { 

      use crate::lemmy_client::*;

      #[cfg(feature = "ssr")]
      let jwt = {
        use actix_session::Session;
        use leptos_actix::extract;
        extract(|session: Session| async move {
          session.get::<String>("jwt")
        })
        .await??
      };
    
      #[cfg(not(feature = "ssr"))]
      let jwt = {
        use wasm_cookies::get;
        get("jwt").and_then(Result::ok)
      };
    
      (Fetch {}).get_site(jwt).await
      // get_site().await 
    },
    QueryOptions {
      resource_option: ResourceOption::Blocking,
      ..QueryOptions::default()
    },
  )
}
