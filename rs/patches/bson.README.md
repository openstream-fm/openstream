# bson crate patch

This patch to the `bson` crate is about setting the `(De)Serializer::is_human_readable` to return `false` by default for all `(De)Serializers`

This patch will probably be merged in next breaking version of `bson` crate in crates.io

This way we can diferentiate between JSON (De)Serializer and BSON (De)Serializer for, for example, `DateTime` and `Binary` representations

If this patch is not used `mongodb` crate will use `is_human_readable() == false` for `Serializer` and `is_human_readable() == true` for `Deserializer`