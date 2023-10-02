use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Connection {
    pub ports: Vec<Port>,
}

impl Eq for Connection {}
impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.ports.len() == other.ports.len()
            && self
                .ports
                .iter()
                .zip(other.ports.iter())
                .all(|(a, b)| a.pins == b.pins)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Port {
    state: State,
    pub pins: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Connected,
    Disconnected,
}

#[derive(Debug)]
pub struct UnableToParseConnection;
impl FromStr for Connection {
    type Err = UnableToParseConnection;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ports = s
            .split("/")
            .map(|x| {
                let pins = x.trim().parse().map_err(|_| UnableToParseConnection)?;
                let state = if pins == 0 {
                    State::Connected
                } else {
                    State::Disconnected
                };
                Ok(Port { state, pins })
            })
            .collect::<Result<Vec<Port>, UnableToParseConnection>>()?;
        Ok(Connection { ports })
    }
}

#[derive(Debug)]
pub struct ImproperConnection;

impl Connection {
    pub fn can_connect(&self, other: &Connection) -> bool {
        for port_self in Self::ports_by_state(self, State::Disconnected) {
            for port_other in Self::ports_by_state(other, State::Disconnected) {
                if port_self.pins == port_other.pins {
                    return true;
                }
            }
        }
        false
    }

    pub fn connect(&mut self, other: &mut Connection) -> Result<(), ImproperConnection> {
        for port_self in Self::ports_by_state_mut(self, State::Disconnected) {
            for port_other in Self::ports_by_state_mut(other, State::Disconnected) {
                if port_self.pins == port_other.pins {
                    port_self.state = State::Connected;
                    port_other.state = State::Connected;
                    return Ok(());
                }
            }
        }

        Err(ImproperConnection)
    }

    fn ports_by_state(conn: &Connection, state: State) -> Vec<&Port> {
        conn.ports.iter().filter(|p| p.state == state).collect()
    }

    fn ports_by_state_mut(conn: &mut Connection, state: State) -> Vec<&mut Port> {
        conn.ports.iter_mut().filter(|p| p.state == state).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_connect() {
        let conn_a = Connection {
            ports: vec![
                Port {
                    state: State::Disconnected,
                    pins: 1,
                },
                Port {
                    state: State::Connected,
                    pins: 10,
                },
            ],
        };

        let conn_b = Connection {
            ports: vec![
                Port {
                    state: State::Disconnected,
                    pins: 10,
                },
                Port {
                    state: State::Disconnected,
                    pins: 5,
                },
            ],
        };

        assert!(!conn_a.can_connect(&conn_b));
        assert!(!conn_b.can_connect(&conn_a));

        let conn_a = Connection {
            ports: vec![
                Port {
                    state: State::Disconnected,
                    pins: 1,
                },
                Port {
                    state: State::Disconnected,
                    pins: 10,
                },
            ],
        };

        let conn_b = Connection {
            ports: vec![
                Port {
                    state: State::Disconnected,
                    pins: 10,
                },
                Port {
                    state: State::Disconnected,
                    pins: 5,
                },
            ],
        };

        assert!(conn_a.can_connect(&conn_b));
        assert!(conn_b.can_connect(&conn_a));
    }
}
