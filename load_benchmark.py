import os
import pathlib

from gymbag_rs import proccess_section_pickled, read_gymbag_to_game_states

import multiprocessing
import pickle

WORKERS = 12

pool = multiprocessing.Pool(WORKERS)

files = [
    str(file)
    for file in pathlib.Path("./recordings/").iterdir()
    if file.is_file() and file.name.endswith(".gymbag2")
]


file_groups = [files[i::12] for i in range(WORKERS)]

results = pool.map(proccess_section_pickled, file_groups)

output = []

for result in results:
    for r in result:
        output.append(pickle.loads(r))

print(len(output))
