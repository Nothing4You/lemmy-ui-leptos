use crate::{
  i18n::*,
  ui::components::site_state_provider::{SiteStateContext, SiteStateProvider},
};
use leptos::*;
use leptos_icons::*;
use leptos_router::*;

#[server(LogoutAction, "serverfn")]
pub async fn logout() -> Result<(), ServerFnError> {
  use actix_session::Session;
  use leptos_actix::extract;

  extract(|session: Session| async move {
    // TODO: Will have to make API call to delete session stored in DB once that feature is implemented on the server
    session.purge();
  })
  .await
}

#[component]
pub fn TopNav() -> impl IntoView {
  let get_site_resource = use_context::<SiteStateContext>();
  let i18n = use_i18n();
  let logged_in = Signal::derive(move || {
    let resource = get_site_resource.unwrap();
    !resource.loading()() && resource().unwrap().unwrap().my_user.is_some()
  });

  let logout_action = create_server_action::<LogoutAction>();

  view! {
    <nav class="navbar container mx-auto">
      <div class="navbar-start">
        <ul class="menu menu-horizontal flex-nowrap">
          <li>
            <A href="/" class="text-xl whitespace-nowrap">
              "Brand from env"
            </A>
          </li>
          <li>
            <A href="/communities" class="text-md">
              {t!(i18n, communities)}
            </A>
          </li>
          <li>
            <A href="/create_post" class="text-md">
              {t!(i18n, create_post)}
            </A>
          </li>
          <li>
            <A href="/create_community" class="text-md">
              {t!(i18n, create_community)}
            </A>
          </li>
          <li>
            <a href="join-lemmy.org/donate">
              <span title=t!(i18n, donate)>
                <Icon icon=Icon::from(ChIcon::ChHeart) class="h-6 w-6"/>
              </span>
            </a>
          </li>
        </ul>
      </div>
      <div class="navbar-end">
        <ul class="menu menu-horizontal flex-nowrap">
          <li>
            <A href="/search">
              <span title=t!(i18n, search)>
                <Icon icon=Icon::from(ChIcon::ChSearch) class="h-6 w-6"/>
              </span>
            </A>
          </li>
          <li>
            <Show
              when=logged_in
              fallback=move || view!{<A href="/login">{t!(i18n, login)}</A>}
              >
              <ActionForm action=logout_action>
                <button type="submit">{t!(i18n, logout)}</button>
              </ActionForm>
            </Show>
          </li>
        </ul>
      </div>
    </nav>
  }
}

#[component]
pub fn BottomNav() -> impl IntoView {
  let i18n = use_i18n();
  view! {
    <footer class="sticky bottom-0">
      <nav class="container navbar mx-auto">
        <div class="navbar-start"></div>
        <div class="navbar-end ">
          <ul class="menu menu-horizontal flex-nowrap">
            <li>
              <a href="github.com/LemmyNet/lemmy-ui-leptos/releases" class="text-md">
                "f/e from env"
              </a>
            </li>
            <li>
              <a href="github.com/LemmyNet/lemmy/releases" class="text-md">
                "b/e from env"
              </a>
            </li>
            <li>
              <A href="/modlog" class="text-md">
                {t!(i18n, modlog)}
              </A>
            </li>
            <li>
              <A href="/instances" class="text-md">
                {t!(i18n, instances)}
              </A>
            </li>
            <li>
              <a href="join-lemmy.org/docs/en/index.html" class="text-md">
                {t!(i18n, docs)}
              </a>
            </li>
            <li>
              <a href="github.com/LemmyNet" class="text-md">
                {t!(i18n, code)}
              </a>
            </li>
            <li>
              <a href="join-lemmy.org" class="text-md">
                "join-lemmy.org"
              </a>
            </li>
          </ul>
        </div>
      </nav>
    </footer>
  }
}
