// @generated
impl serde::Serialize for CreateProjectRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("demeter.ops.v1alpha.CreateProjectRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateProjectRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateProjectRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct demeter.ops.v1alpha.CreateProjectRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateProjectRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateProjectRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("demeter.ops.v1alpha.CreateProjectRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateProjectResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.namespace.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("demeter.ops.v1alpha.CreateProjectResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.namespace.is_empty() {
            struct_ser.serialize_field("namespace", &self.namespace)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateProjectResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "namespace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Namespace,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "namespace" => Ok(GeneratedField::Namespace),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateProjectResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct demeter.ops.v1alpha.CreateProjectResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateProjectResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut namespace__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Namespace => {
                            if namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            namespace__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateProjectResponse {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    namespace: namespace__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("demeter.ops.v1alpha.CreateProjectResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateProjectSecretRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.project_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("demeter.ops.v1alpha.CreateProjectSecretRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateProjectSecretRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "project_id",
            "projectId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ProjectId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateProjectSecretRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct demeter.ops.v1alpha.CreateProjectSecretRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateProjectSecretRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut project_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateProjectSecretRequest {
                    name: name__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("demeter.ops.v1alpha.CreateProjectSecretRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateProjectSecretResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("demeter.ops.v1alpha.CreateProjectSecretResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateProjectSecretResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Key,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateProjectSecretResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct demeter.ops.v1alpha.CreateProjectSecretResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateProjectSecretResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateProjectSecretResponse {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("demeter.ops.v1alpha.CreateProjectSecretResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateResourceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_id.is_empty() {
            len += 1;
        }
        if !self.kind.is_empty() {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("demeter.ops.v1alpha.CreateResourceRequest", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.kind.is_empty() {
            struct_ser.serialize_field("kind", &self.kind)?;
        }
        if !self.data.is_empty() {
            struct_ser.serialize_field("data", &self.data)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateResourceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
            "kind",
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
            Kind,
            Data,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "kind" => Ok(GeneratedField::Kind),
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateResourceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct demeter.ops.v1alpha.CreateResourceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateResourceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                let mut kind__ = None;
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateResourceRequest {
                    project_id: project_id__.unwrap_or_default(),
                    kind: kind__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("demeter.ops.v1alpha.CreateResourceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateResourceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.kind.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("demeter.ops.v1alpha.CreateResourceResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.kind.is_empty() {
            struct_ser.serialize_field("kind", &self.kind)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateResourceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "kind",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Kind,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "kind" => Ok(GeneratedField::Kind),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateResourceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct demeter.ops.v1alpha.CreateResourceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateResourceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateResourceResponse {
                    id: id__.unwrap_or_default(),
                    kind: kind__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("demeter.ops.v1alpha.CreateResourceResponse", FIELDS, GeneratedVisitor)
    }
}
