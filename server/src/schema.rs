table! {
    tenants (id) {
        id -> Int8,
        app_id -> Int8,
        github_login -> Varchar,
        github_id -> Int8,
        block_list -> Text,
        captcha -> Bool,
    }
}

table! {
    wechat_works (id) {
        id -> Int8,
        tenant_id -> Int8,
        corp_id -> Varchar,
        agent_id -> Int8,
        secret -> Varchar,
        bot_token -> Text,
        chat_id -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    tenants,
    wechat_works,
);
