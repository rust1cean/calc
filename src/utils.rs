use std::ops::{Add, Div, Mul, Sub};

pub enum Task {
    Immut(fn()),
    Mut(fn(&mut App)),
    Read(fn(&App)),
}

pub struct App {
    state: State,
    store: Store,
    systems: System,
    tasks: System,
}

impl App {
    pub fn new() -> Self {
        let state = State::Run;
        let store = Store::new();
        let systems = System::new();
        let tasks = System::new();

        Self {
            state,
            store,
            systems,
            tasks,
        }
    }

    pub fn run(&mut self) {
        while self.state == State::Run {
            if self.has_tasks() {
                let task = self.rm_task(0);
                self.complete_task(task);
            } else if self.has_systems() {
                let task = self.rm_system(0);
                self.complete_task(task);
            }
        }
    }

    pub fn stop(&mut self) {
        self.state = State::Die;
    }
}

impl App {
    pub fn system(&mut self, system: Task) -> &mut Self {
        self.systems.task(system);
        self
    }

    pub fn rm_system(&mut self, idx: usize) -> Task {
        self.systems.rm(0)
    }

    pub fn has_systems(&self) -> bool {
        self.systems.len() > 0
    }

    pub fn clear_systems(&mut self) {
        self.systems.clear();
    }
}

impl App {
    pub fn task(&mut self, task: Task) -> &mut Self {
        self.tasks.task(task);
        self
    }

    pub fn rm_task(&mut self, idx: usize) -> Task {
        self.tasks.rm(0)
    }

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }

    pub fn complete_task(&mut self, task: Task) {
        match task {
            Task::Immut(task) => task(),
            Task::Mut(task) => task(self),
            Task::Read(task) => task(self),
        }
    }

    pub fn has_tasks(&self) -> bool {
        self.tasks.len() > 0
    }
}

impl App {
    pub fn immut_system(&mut self, system: fn()) -> &mut Self {
        self.systems.task(Task::Immut(system));
        self
    }

    pub fn mut_system(&mut self, system: fn(&mut Self)) -> &mut Self {
        self.systems.task(Task::Mut(system));
        self
    }

    pub fn read_system(&mut self, system: fn(&Self)) -> &mut Self {
        self.systems.task(Task::Read(system));
        self
    }

    pub fn immut_task(&mut self, task: fn()) -> &mut Self {
        self.tasks.task(Task::Immut(task));
        self
    }

    pub fn mut_task(&mut self, task: fn(&mut Self)) -> &mut Self {
        self.tasks.task(Task::Mut(task));
        self
    }

    pub fn read_task(&mut self, task: fn(&Self)) -> &mut Self {
        self.tasks.task(Task::Read(task));
        self
    }
}

impl App {
    pub fn push_data(&mut self, id: Id, data: Data) {
        self.store.push(id, data)
    }

    pub fn read_data(&self, id: usize) -> Option<&(Id, Data)> {
        self.store.read(id)
    }

    pub fn rm_data(&mut self, id: usize) -> Option<(usize, Data)> {
        self.store.rm(id)
    }

    pub fn edit_data(&mut self, id: usize, data: Data) {
        self.store.edit(id, data);
    }
}

#[derive(PartialEq)]
pub enum State {
    Run,
    Die,
}

pub type Id = usize;

#[derive(Debug)]
pub enum Data {
    I32(i32),
    String(String),
    // Str(&str),
}

impl Add for Data {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Data::I32(first), Data::I32(second)) => Some(Data::I32(first + second)),
            _ => None,
        }
    }
}

impl Sub for Data {
    type Output = Option<Self>;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Data::I32(first), Data::I32(second)) => Some(Data::I32(first - second)),
            _ => None,
        }
    }
}

impl Mul for Data {
    type Output = Option<Self>;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Data::I32(first), Data::I32(second)) => Some(Data::I32(first * second)),
            _ => None,
        }
    }
}

impl Div for Data {
    type Output = Option<Self>;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Data::I32(first), Data::I32(second)) => Some(Data::I32(first / second)),
            _ => None,
        }
    }
}

pub struct Store(Vec<(Id, Data)>);

impl Store {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, id: Id, data: Data) {
        self.0.push((id, data));
    }

    pub fn position(&self, id: usize) -> Option<usize> {
        self.0.iter().position(|(i, _)| *i == id)
    }

    pub fn insert(&mut self, idx: usize, new: (Id, Data)) {
        self.0.insert(idx, new);
    }

    pub fn read(&self, id: usize) -> Option<&(Id, Data)> {
        self.0.iter().find(|(i, _)| *i == id)
    }

    pub fn rm(&mut self, id: usize) -> Option<(Id, Data)> {
        if let Some(idx) = self.position(id) {
            return Some(self.0.remove(idx));
        }
        None
    }

    pub fn edit(&mut self, id: usize, data: Data) {
        if let Some(idx) = self.position(id) {
            self.rm(idx);
            self.insert(idx, (id, data));
        }
    }
}

pub struct System(pub Vec<Task>);

impl System {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn task(&mut self, task: Task) {
        self.0.push(task);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn rm(&mut self, idx: usize) -> Task {
        self.0.remove(idx)
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}
