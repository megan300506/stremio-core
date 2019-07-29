use super::addons::*;
use crate::state_types::msg::Internal::*;
use crate::state_types::*;
use crate::types::addons::{AggrRequest, ResourceRequest, ResourceResponse};
use crate::types::MetaPreview;
use serde_derive::*;
use itertools::*;

#[derive(Debug, Clone, Default, Serialize)]
pub struct CatalogGrouped {
    pub groups: Vec<ItemsGroup<Vec<MetaPreview>>>,
}
impl<Env: Environment + 'static> UpdateWithCtx<Ctx<Env>> for CatalogGrouped {
    fn update(&mut self, ctx: &Ctx<Env>, msg: &Msg) -> Effects {
        match msg {
            Msg::Action(Action::Load(ActionLoad::CatalogGrouped { extra })) => {
                let (groups, effects) = addon_aggr_new::<Env, _>(
                    &ctx.content.addons,
                    &AggrRequest::AllCatalogs { extra },
                );
                self.groups = groups;
                effects
            }
            _ => addon_aggr_update(&mut self.groups, msg),
        }
    }
}

//
// Filtered catalogs
//
use crate::types::addons::ManifestCatalog;
#[derive(Debug, Default, Clone, Serialize)]
pub struct CatalogFiltered {
    pub item_pages: Vec<Loadable<Vec<MetaPreview>, String>>,
    pub types: Vec<String>,
    pub catalogs: Vec<ManifestCatalog>,
    pub selected: Option<ResourceRequest>,
    // @TODO types to be { load_msg, is_selected, type_name }
    // @TODO catalogs to be { load_msg, is_selected, name, type }
    // is_selected will be whether the path matches selected, excluding the `skip` (page)
    // @TODO: extra (filters); there should be .extra, of all selectable extra props; consider that
    // some can be defaulted
    // @TODO pagination; this can be done by incrementing skip in the ResourceRequest when requesting
    // the next page; we will have .load_next/.load_prev (Option<ActionLoad>) to go to next/prev
    // page
}
impl<Env: Environment + 'static> UpdateWithCtx<Ctx<Env>> for CatalogFiltered {
    fn update(&mut self, ctx: &Ctx<Env>, msg: &Msg) -> Effects {
        match msg {
            Msg::Action(Action::Load(ActionLoad::CatalogFiltered { resource_req })) => {
                // @TODO catalog by types
                // @TODO pagination
                let addons = &ctx.content.addons;
                self.catalogs = addons
                    .iter()
                    .flat_map(|a| &a.manifest.catalogs)
                    // this will weed out catalogs that require extra props
                    .filter(|cat| cat.is_extra_supported(&[]))
                    // @TODO another filter cause of `selected` with .map_or
                    .cloned()
                    .collect();
                // The alternative to the HashSet is to sort and dedup
                // but we want to preserve the original order in which types appear in
                self.types = self
                    .catalogs
                    .iter()
                    .map(|x| x.type_name.clone())
                    .unique()
                    .collect();
                self.item_pages = vec![Loadable::Loading];
                self.selected = Some(resource_req.to_owned());
                Effects::one(addon_get::<Env>(&resource_req))
            }
            Msg::Internal(AddonResponse(req, result))
                if Some(req) == self.selected.as_ref()
                    && self.item_pages.last() == Some(&Loadable::Loading) =>
            {
                self.item_pages[0] = match result.as_ref() {
                    Ok(ResourceResponse::Metas { metas }) => Loadable::Ready(metas.to_owned()),
                    Ok(_) => Loadable::Err(UNEXPECTED_RESP_MSG.to_owned()),
                    Err(e) => Loadable::Err(e.to_string()),
                };
                Effects::none()
            }
            _ => Effects::none().unchanged(),
        }
    }
}
