import os, cv2
from pathlib import Path
from file_formats.definitions import ROOT_DIR


def main():
    # define path to root
    
    print(ROOT_DIR)

    # initialize a camera object on port 0
    camera = cv2.VideoCapture(0)

    # generate the images used for experiments (using a for loop with no delay for least variance)
    images = [[*camera.read(), i] for i in range(5)]

    # save the images to disk with no compression
    [
        cv2.imwrite(
            f"{ROOT_DIR}/data/images/{index}.png",
            image,
            [cv2.IMWRITE_PNG_COMPRESSION, 0],
        )
        for result, image, index in images
        if result
    ]


if __name__ == "__main__":
    main()
