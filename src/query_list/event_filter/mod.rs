use crate::query_list::Comparison;
use std::fmt;

pub mod computer;
pub mod data;
pub mod event;
pub mod level;
pub mod provider;
pub mod user;

#[derive(Clone)]
pub enum SystemFilter {
    Computer(computer::Computer),
    EventID(event::Event),
    Level(level::Level),
    Provider(provider::Provider),
    User(user::User),
}

#[derive(Clone)]
pub struct EventDataFilter {
    name: data::Name,
    value: data::Value,
}

impl EventDataFilter {
    pub fn new(name: String, value: String) -> EventDataFilter {
        EventDataFilter {
            name: data::Name::new(name),
            value: data::Value::new(value),
        }
    }
}

#[derive(Clone)]
pub enum EventFilter {
    System(SystemFilter),
    EventData(EventDataFilter),
}

impl EventFilter {
    pub fn computer(name: String) -> EventFilter {
        EventFilter::System(SystemFilter::Computer(computer::Computer::new(name)))
    }

    pub fn event(id: u32) -> EventFilter {
        EventFilter::System(SystemFilter::EventID(event::Event::new(id)))
    }

    pub fn level(level: u32, comparison: Comparison) -> EventFilter {
        EventFilter::System(SystemFilter::Level(level::Level::new(level, comparison)))
    }

    pub fn provider(name: String) -> EventFilter {
        EventFilter::System(SystemFilter::Provider(provider::Provider::new(name)))
    }

    pub fn user(sid: String) -> EventFilter {
        EventFilter::System(SystemFilter::User(user::User::new(sid)))
    }

    pub fn event_data(name: String, value: String) -> EventFilter {
        EventFilter::EventData(EventDataFilter::new(name, value))
    }
}

impl fmt::Display for EventFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventFilter::System(item) => write!(f, "{}", item),
            EventFilter::EventData(item) => write!(f, "{}", item),
        }
    }
}

impl fmt::Display for SystemFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SystemFilter::Computer(item) => write!(f, "{}", item),
            SystemFilter::EventID(item) => write!(f, "{}", item),
            SystemFilter::Level(item) => write!(f, "{}", item),
            SystemFilter::Provider(item) => write!(f, "{}", item),
            SystemFilter::User(item) => write!(f, "{}", item),
        }
    }
}

impl fmt::Display for EventDataFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} and {})", self.name, self.value)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn complex_query() {
        use crate::prelude::*;
        let conditions = vec![
            Condition::filter(EventFilter::event(4624)),
            Condition::filter(EventFilter::event(1102)),
        ];

        let ql = QueryList::new()
            .with_query(
                Query::new()
                    .item(
                        QueryItem::selector("Security".to_owned())
                            .system_conditions(Condition::or(conditions))
                            .build(),
                    )
                    .item(
                        QueryItem::suppressor("Security".to_owned())
                            .system_conditions(Condition::filter(EventFilter::event(4624)))
                            .event_conditions(Condition::filter(EventFilter::event_data(
                                "TargetUserName".to_owned(),
                                "SYSTEM".to_owned(),
                            )))
                            .build(),
                    )
                    .query(),
            )
            .build();
        assert_eq!(
            ql.to_string(),
            r#"<QueryList>
<Query Id="0">
<Select Path="Security">
*[System[((EventID = 4624) or (EventID = 1102))]]
</Select>
<Suppress Path="Security">
*[System[(EventID = 4624)]]
and
*[EventData[((Data[@Name = 'TargetUserName'] and Data = 'SYSTEM'))]]
</Suppress>
</Query>
</QueryList>"#
        );
    }
}
