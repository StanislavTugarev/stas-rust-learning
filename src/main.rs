// trait object coercion
// &T -> &dyn Trait // T implements Trait

trait Command {
    fn execute(&self);
}

struct TurnOnLight {
    room: String,
}

struct AdjustThermostat {
    temperature: f32,
}

struct TurnOnBlack {
    room: String,
}

impl Command for TurnOnLight {
    fn execute(&self) {
        println!("Turning on lights in the {}", self.room)
    }
}

impl Command for AdjustThermostat {
    fn execute(&self) {
        println!("Setting thermostat to {}C", self.temperature)
    }
}

impl Command for TurnOnBlack {
    fn execute(&self) {
        println!("Turn Black in the {}", self.room)
    }
}

fn execute_command(command: &dyn Command) {
    command.execute();
}

fn execute_command_1<T: Command>(command: &T) {
    command.execute();
}

fn main() {
    let light_cmd = TurnOnLight {
        room: "Living Room".to_string(),
    };

    let thermostat_cmd = AdjustThermostat { temperature: 25.5 };

    // trait object coercion: &T -> &dyn Command
    execute_command(&light_cmd);
    execute_command(&thermostat_cmd);

    execute_command_1(&light_cmd);
    execute_command_1(&thermostat_cmd);

    let command_queue: Vec<&dyn Command> = vec![&light_cmd, &thermostat_cmd];
}
