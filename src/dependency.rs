use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum UpdateType {
    Always,
    Never,
    OnChange
}

impl UpdateType {
    pub fn from_str(value: &str) -> Self {
    	match value {
    	    "always" => Self::Always,
            "never" => Self::Never,
            &_ => Self::OnChange
    	}
    }
}

impl Serialize for UpdateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
    	match self {
    	    Self::Always => serializer.serialize_str("always"),
    	    Self::Never => serializer.serialize_str("never"),
    	    Self::OnChange => serializer.serialize_str("on_change")
    	}
    }
}

impl<'de> Deserialize<'de> for UpdateType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
	{
		let value = String::deserialize(deserializer)?;

        match value.as_str() {
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            &_ => Ok(Self::OnChange)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dependency {
	pub name: String,
	pub uri: String,
	pub path: String,
	pub local: bool,
	pub update: UpdateType
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
	pub dependencies: Vec<Dependency>
}

impl Configuration {
	pub fn add_dependency(&mut self, name: &str, uri: &str, path: &str, local: &bool, update: &str) {
		if let Some(dependency) = self.dependencies.iter_mut().find(|d| d.name == name) {
			dependency.uri = String::from(uri);
		} else {
			self.dependencies.push(Dependency {
				name: String::from(name),
				uri: String::from(uri),
				path: String::from(path),
				local: *local,
				update: UpdateType::from_str(update)
			});
		}
	}

	pub fn remove_dependency(&mut self, name: &str) -> Option<Dependency> {
	    let dependency_index = self.dependencies.iter().position(|dependency| dependency.name == name)?;
		let dependency = self.dependencies.remove(dependency_index);

	    Some(dependency)
	}
}