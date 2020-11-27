use radmin::{
    crate_authors, crate_description, crate_name, crate_version, Application, ServerError,
};
use radmin_contacts::ContactsModule;

fn main() -> Result<(), ServerError> {
    let app = Application::default()
        .name(crate_name!())
        .version(crate_version!())
        .description(crate_description!())
        .author(crate_authors!())
        .add_module_default::<ContactsModule>();
    radmin::run(app)
}
