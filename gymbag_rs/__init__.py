from .gymbag_rs import *

__all__ = (
    "read_gymbag_to_game_states",
    "Game",
    "GameState",
    "Task",
    "PosistionData",
    "Vector2",
    "process_section",
    "proccess_section_pickled",
)


def process_section(files: list[str]):
    return read_many_gymbag_to_xs_ys(files)

def proccess_section_pickled(files: list[str]):
    return read_many_gymbag_to_xs_ys_pickled(files)
