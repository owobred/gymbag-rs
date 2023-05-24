use pyo3::{prelude::*, types::PyBytes};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{decode::read_gymbag as read_gymbag_rust, frame, game};

#[pyfunction]
pub fn read_gymbag_to_game_states(file: String) -> PyResult<Game> {
    let frames = read_gymbag_rust(file);
    let game = game::Game::from_frames(frames);
    Ok(game.into())
}

#[pymodule]
pub fn gymbag_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_gymbag_to_game_states, m)?)?;
    m.add_class::<Game>()?;
    m.add_class::<GameState>()?;
    m.add_class::<Vector2>()?;
    m.add_class::<TaskData>()?;
    m.add_class::<PositionData>()?;

    #[pyfn(m)]
    pub fn read_gymbag_to_xs_ys(file: String) -> PyResult<(Vec<Vec<Vec<f64>>>, Vec<Vec<Vec<f64>>>)> {
        let frames = read_gymbag_rust(file);
        let game = game::Game::from_frames(frames);
        Ok((game.get_x(), game.get_y()))
    }

    #[pyfn(m)]
    pub fn read_many_gymbag_to_xs_ys(files: Vec<String>) -> PyResult<Vec<(Vec<Vec<Vec<f64>>>, Vec<Vec<Vec<f64>>>)>> {
        let x = files.into_par_iter().map(|file| {
            let frames = read_gymbag_rust(file);
            let game = game::Game::from_frames(frames);
    
            (game.get_x(), game.get_y())
        }).collect::<Vec<_>>();
        Ok(x)
    }

    #[pyfn(m)]
    pub fn read_many_gymbag_to_xs_ys_pickled<'py>(py: Python<'py>, files: Vec<String>) -> PyResult<Vec<&PyBytes>> {
        let x = files.into_par_iter().map(|file| {
            let frames = read_gymbag_rust(file);
            let game = game::Game::from_frames(frames);
    
            serde_pickle::to_vec(&(game.get_x(), game.get_y()), Default::default()).expect("failed to pickle")
        }).collect::<Vec<_>>();
        Ok(x.into_iter().map(|bytes| PyBytes::new(py, &bytes)).collect::<Vec<_>>())
    }

    Ok(())
}

impl From<game::Game> for Game {
    fn from(value: game::Game) -> Self {
        Self {
            states: value
                .states
                .into_iter()
                .map(|state| state.into())
                .collect::<Vec<_>>(),
        }
    }
}

impl From<game::GameState> for GameState {
    fn from(value: game::GameState) -> Self {
        Self {
            data: value.data,
            header: value.header,
            last_velocity: value.last_velocity.into(),
        }
    }
}

impl From<frame::Vector2> for Vector2 {
    fn from(value: frame::Vector2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct Game {
    #[pyo3(get)]
    pub states: Vec<GameState>,
}

#[pymethods]
impl Game {
    #[getter]
    fn state_count(&self) -> usize {
        self.states.len()
    }
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct GameState {
    // these don't have a python class due to not being done by prost,
    // i might be able to make them accessible?
    data: frame::Frame,
    header: frame::HeaderFrame,
    #[pyo3(get)]
    last_velocity: Vector2,
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct Vector2 {
    #[pyo3(get)]
    pub x: f32,
    #[pyo3(get)]
    pub y: f32,
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct TaskData {
    #[pyo3(get)]
    pub id: i32,
    #[pyo3(get)]
    pub r#type: i32,
    #[pyo3(get)]
    pub consoles_of_interest: Vec<PositionData>,
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct PositionData {
    #[pyo3(get)]
    pub total_distance: f32,
    #[pyo3(get)]
    pub next_node_position: Option<Vector2>,
    #[pyo3(get)]
    pub next_node_offset: Option<Vector2>,
}