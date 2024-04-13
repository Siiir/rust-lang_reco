import os
from pathlib import Path

def empty_directories(base_path, directories):
    for directory in directories:
        path = Path(base_path) / directory
        for subdirectory in path.iterdir():
            if subdirectory.is_dir():
                for file in subdirectory.glob('*.txt'):
                    file.unlink()  # Deletes each file

if __name__ == "__main__":
    base_path = '.'  # Adjust this to the path of your project's data folder
    directories = ['test', 'train']
    empty_directories(base_path, directories)
    print("Directories have been emptied.")

