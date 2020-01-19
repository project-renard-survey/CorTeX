table! {
    /// Representation of the `corpora` table.
    ///
    /// (Automatically generated by Diesel.)
    corpora (id) {
        /// The `id` column of the `corpora` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `path` column of the `corpora` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        path -> Varchar,
        /// The `name` column of the `corpora` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
        /// The `complex` column of the `corpora` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        complex -> Bool,
        /// The `description` column of the `corpora` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        description -> Text,
    }
}

table! {
    /// Representation of the `daemons` table.
    ///
    /// (Automatically generated by Diesel.)
    daemons (id) {
        /// The `id` column of the `daemons` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `pid` column of the `daemons` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        pid -> Int4,
        /// The `first_seen` column of the `daemons` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        first_seen -> Timestamp,
        /// The `last_seen` column of the `daemons` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        last_seen -> Timestamp,
        /// The `name` column of the `daemons` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
    }
}

table! {
    /// Representation of the `dependencies` table.
    ///
    /// (Automatically generated by Diesel.)
    dependencies (master, foundation) {
        /// The `master` column of the `dependencies` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        master -> Int4,
        /// The `foundation` column of the `dependencies` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        foundation -> Int4,
    }
}

table! {
    /// Representation of the `historical_runs` table.
    ///
    /// (Automatically generated by Diesel.)
    historical_runs (id) {
        /// The `id` column of the `historical_runs` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `service_id` column of the `historical_runs` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        service_id -> Int4,
        /// The `corpus_id` column of the `historical_runs` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        corpus_id -> Int4,
        /// The `total` column of the `historical_runs` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        total -> Int4,
        /// The `invalid` column of the `historical_runs` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        invalid -> Int4,
        /// The `fatal` column of the `historical_runs` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fatal -> Int4,
        /// The `error` column of the `historical_runs` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        error -> Int4,
        /// The `warning` column of the `historical_runs` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        warning -> Int4,
        /// The `no_problem` column of the `historical_runs` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        no_problem -> Int4,
        /// The `in_progress` column of the `historical_runs` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        in_progress -> Int4,
        /// The `start_time` column of the `historical_runs` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        start_time -> Timestamp,
        /// The `end_time` column of the `historical_runs` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        end_time -> Nullable<Timestamp>,
        /// The `owner` column of the `historical_runs` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        owner -> Varchar,
        /// The `description` column of the `historical_runs` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        description -> Text,
    }
}

table! {
    /// Representation of the `log_errors` table.
    ///
    /// (Automatically generated by Diesel.)
    log_errors (id) {
        /// The `id` column of the `log_errors` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `task_id` column of the `log_errors` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        task_id -> Int8,
        /// The `category` column of the `log_errors` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        category -> Nullable<Varchar>,
        /// The `what` column of the `log_errors` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        what -> Nullable<Varchar>,
        /// The `details` column of the `log_errors` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        details -> Nullable<Varchar>,
    }
}

table! {
    /// Representation of the `log_fatals` table.
    ///
    /// (Automatically generated by Diesel.)
    log_fatals (id) {
        /// The `id` column of the `log_fatals` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `task_id` column of the `log_fatals` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        task_id -> Int8,
        /// The `category` column of the `log_fatals` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        category -> Nullable<Varchar>,
        /// The `what` column of the `log_fatals` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        what -> Nullable<Varchar>,
        /// The `details` column of the `log_fatals` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        details -> Nullable<Varchar>,
    }
}

table! {
    /// Representation of the `log_infos` table.
    ///
    /// (Automatically generated by Diesel.)
    log_infos (id) {
        /// The `id` column of the `log_infos` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `task_id` column of the `log_infos` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        task_id -> Int8,
        /// The `category` column of the `log_infos` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        category -> Nullable<Varchar>,
        /// The `what` column of the `log_infos` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        what -> Nullable<Varchar>,
        /// The `details` column of the `log_infos` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        details -> Nullable<Varchar>,
    }
}

table! {
    /// Representation of the `log_invalids` table.
    ///
    /// (Automatically generated by Diesel.)
    log_invalids (id) {
        /// The `id` column of the `log_invalids` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `task_id` column of the `log_invalids` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        task_id -> Int8,
        /// The `category` column of the `log_invalids` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        category -> Nullable<Varchar>,
        /// The `what` column of the `log_invalids` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        what -> Nullable<Varchar>,
        /// The `details` column of the `log_invalids` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        details -> Nullable<Varchar>,
    }
}

table! {
    /// Representation of the `log_warnings` table.
    ///
    /// (Automatically generated by Diesel.)
    log_warnings (id) {
        /// The `id` column of the `log_warnings` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `task_id` column of the `log_warnings` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        task_id -> Int8,
        /// The `category` column of the `log_warnings` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        category -> Nullable<Varchar>,
        /// The `what` column of the `log_warnings` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        what -> Nullable<Varchar>,
        /// The `details` column of the `log_warnings` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        details -> Nullable<Varchar>,
    }
}

table! {
    /// Representation of the `services` table.
    ///
    /// (Automatically generated by Diesel.)
    services (id) {
        /// The `id` column of the `services` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `name` column of the `services` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
        /// The `version` column of the `services` table.
        ///
        /// Its SQL type is `Float4`.
        ///
        /// (Automatically generated by Diesel.)
        version -> Float4,
        /// The `inputformat` column of the `services` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        inputformat -> Varchar,
        /// The `outputformat` column of the `services` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        outputformat -> Varchar,
        /// The `inputconverter` column of the `services` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        inputconverter -> Nullable<Varchar>,
        /// The `complex` column of the `services` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        complex -> Bool,
        /// The `description` column of the `services` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        description -> Text,
    }
}

table! {
    /// Representation of the `tasks` table.
    ///
    /// (Automatically generated by Diesel.)
    tasks (id) {
        /// The `id` column of the `tasks` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `service_id` column of the `tasks` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        service_id -> Int4,
        /// The `corpus_id` column of the `tasks` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        corpus_id -> Int4,
        /// The `status` column of the `tasks` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        status -> Int4,
        /// The `entry` column of the `tasks` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        entry -> Varchar,
    }
}

table! {
    /// Representation of the `user_actions` table.
    ///
    /// (Automatically generated by Diesel.)
    user_actions (id) {
        /// The `id` column of the `user_actions` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `user_id` column of the `user_actions` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `corpus_id` column of the `user_actions` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        corpus_id -> Nullable<Int4>,
        /// The `service_id` column of the `user_actions` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        service_id -> Nullable<Int4>,
        /// The `action_counter` column of the `user_actions` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        action_counter -> Int4,
        /// The `last_timestamp` column of the `user_actions` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        last_timestamp -> Timestamp,
        /// The `location` column of the `user_actions` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        location -> Text,
        /// The `description` column of the `user_actions` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        description -> Text,
    }
}

table! {
    /// Representation of the `user_permissions` table.
    ///
    /// (Automatically generated by Diesel.)
    user_permissions (id) {
        /// The `id` column of the `user_permissions` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `user_id` column of the `user_permissions` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `corpus_id` column of the `user_permissions` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        corpus_id -> Nullable<Int4>,
        /// The `service_id` column of the `user_permissions` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        service_id -> Nullable<Int4>,
        /// The `owner` column of the `user_permissions` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        owner -> Bool,
        /// The `developer` column of the `user_permissions` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        developer -> Bool,
        /// The `viewer` column of the `user_permissions` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        viewer -> Bool,
    }
}

table! {
    /// Representation of the `users` table.
    ///
    /// (Automatically generated by Diesel.)
    users (id) {
        /// The `id` column of the `users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `display` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        display -> Text,
        /// The `email` column of the `users` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        email -> Varchar,
        /// The `first_seen` column of the `users` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        first_seen -> Timestamp,
        /// The `last_seen` column of the `users` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        last_seen -> Timestamp,
        /// The `admin` column of the `users` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        admin -> Bool,
    }
}

table! {
    /// Representation of the `worker_metadata` table.
    ///
    /// (Automatically generated by Diesel.)
    worker_metadata (id) {
        /// The `id` column of the `worker_metadata` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `service_id` column of the `worker_metadata` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        service_id -> Int4,
        /// The `last_dispatched_task_id` column of the `worker_metadata` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        last_dispatched_task_id -> Int8,
        /// The `last_returned_task_id` column of the `worker_metadata` table.
        ///
        /// Its SQL type is `Nullable<Int8>`.
        ///
        /// (Automatically generated by Diesel.)
        last_returned_task_id -> Nullable<Int8>,
        /// The `total_dispatched` column of the `worker_metadata` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        total_dispatched -> Int4,
        /// The `total_returned` column of the `worker_metadata` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        total_returned -> Int4,
        /// The `first_seen` column of the `worker_metadata` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        first_seen -> Timestamp,
        /// The `session_seen` column of the `worker_metadata` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        session_seen -> Nullable<Timestamp>,
        /// The `time_last_dispatch` column of the `worker_metadata` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        time_last_dispatch -> Timestamp,
        /// The `time_last_return` column of the `worker_metadata` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        time_last_return -> Nullable<Timestamp>,
        /// The `name` column of the `worker_metadata` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    corpora,
    daemons,
    dependencies,
    historical_runs,
    log_errors,
    log_fatals,
    log_infos,
    log_invalids,
    log_warnings,
    services,
    tasks,
    user_actions,
    user_permissions,
    users,
    worker_metadata,
);
