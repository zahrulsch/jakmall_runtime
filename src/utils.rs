use jakmall_client::fetcher_models::category::Children;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct JakmallCat {
    pub name: String,
    pub slug: String,
}

pub fn recurse_jakmall_cat(chil: Children, collector: &mut Vec<JakmallCat>) {
    let slug = chil.url.split('?').next().unwrap_or(&chil.url);
    let jc = JakmallCat {
        name: chil.name,
        slug: slug.to_string(),
    };

    collector.push(jc);

    if let Some(c) = chil.children {
        for cil in c {
            recurse_jakmall_cat(cil, collector)
        }
    }
}
