from pathlib import Path

ROOT_DIR = Path(__file__).parent.parent
DATA_DIR = ROOT_DIR / "data"
DATA_INPUT_DIR = DATA_DIR / "input"
DATA_OUTPUT_DIR = DATA_DIR / "output"
FACE_DB_DIR = DATA_DIR / "facedb"
HAAR_CASCADES_DIR = ROOT_DIR / "haarcascades"

# creating a list of all directory constants and a list of all directory objects
directory_constants = [variable for variable in dir() if variable.endswith("_DIR")]
directory_objects: list[Path] = [globals()[variable] for variable in directory_constants]


def run():
    # Creating the directories if they don't exist
    for path in directory_objects:
        print(f"{path=}")
        if not path.exists():
            print(f"Creating {path}")
            path.mkdir(parents=True)


if __name__ == "__main__":
    run()
