from typing import Optional, Any

def read_gymbag_to_game_states(path: str) -> Game:
    """Reads a gymbag file and returns a Game object."""

def read_gymbag_to_xs_ys(path: str) -> tuple[list[list[list[float]]], list[list[list[float]]]]:
    """Reads a gymbag file and returns a tuple of xs and ys."""

def read_many_gymbag_to_xs_ys(paths: list[str]) -> list[tuple[list[list[list[float]]], list[list[list[float]]]]]:
    """Reads many gymbag files with multiple threads and returns a list of tuples of xs and ys."""

def read_many_gymbag_to_xs_ys_pickled(paths: list[str]) -> list[bytes]:
    """Reads many gymbag files with multiple threads and returns a list of pickled tuples of xs and ys."""

class Game:
    """A class representing all the states that occured through a game."""

    states: list[GameState]

class GameState:
    """The state of a game on a given frame."""
    data: Any  # these are betterproto messages, i don't know how you want to type these
    header: Any
    last_velocity: Vector2

class Task:
    id: int
    type: int
    consoles_of_interest: list[PosistionData]

class PosistionData:
    total_distance: float
    next_node_position: Optional[Vector2]
    next_node_offset: Optional[Vector2]

class Vector2:
    """A 2D vector."""
    x: float
    y: float