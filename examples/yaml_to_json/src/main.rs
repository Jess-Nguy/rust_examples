use serde_json::{from_reader, to_writer, Value};
mod basic;
use basic::run;
use std::fs::File;
use std::path::Path;
fn main() {
    let yaml_data = r#"
        :submission_id: 09b3828b-b059-4643-b3ed-763d352b56ff
        :submission_version: 0
        :form_id: 8901b824-e638-4e36-a16c-b672d5553725
        :form_version: 5
        :process_id: 55b7f408-85d0-43a2-9c91-8ecb3e1895a4
        :values:
        eventdescription: ''
        photoupload: !ruby/hash:ActiveSupport::HashWithIndifferentAccess
            is_locked: false
            signed_by: ''
            signed_by_with_email: ''
            timestamp: ''
            meaning_of_signature: ''
            value: []
        category: ''
        selectallthatapply: ''
        infobox: ''
        infobox_1: ''
        disposition: ''
        priority: ''
        shortterm: ''
        longterm: ''
        status: ''
        workperformedby: ''
        workperformedon: ''
        timetoperformworkexample0030: ''
        :user_id: 0
        :created_at: "2020-08-10 20:10:18 UTC"
        :updated_at: "2021-01-03 16:05:02 UTC"
    "#;
    run(yaml_data);
}
