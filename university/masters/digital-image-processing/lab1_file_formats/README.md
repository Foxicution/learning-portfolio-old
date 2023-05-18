# Goals
Using Python and OpenCV perform experiments on file formats to determine which of them are the best in image quality vs file size ratio.

# Requirements
- Images and videos need to be taken in the same environment (same lighting, same camera, same distance to the object, etc.)
- When describing the experiments you need to add:
    - Experiment no.
    - Format
    - Image size
    - Length (video)
    - FPS (video)
    - Channel amount
    - Quality measure
    - File size
- Each parameter should contain a separate graph
- Results should be concrete, accurate and use numeric figures as proof. They should be in a numbered list. The numeric figures should be in percentages.

# Tasks
- [x] Use your computer camera to take 5 pictures. 
- [x] Use your computer camera to record 5 videos.
- [x] Save a copy of the videos and images convert to grayscale (overall 10 images and 10 videos)
- [x] Convert images (Formats: bmp, jpg, png)
- [x] Convert videos (Formats: i420 (avi), mp4, DivX (avi), flv, xvideo (avi))
- [x] Measure the size and quality of the files
- [x] Create a table with the results
- [x] Calculate the mean and standard deviation of the results
- [x] Create a graph with the results
- [x] Create a report with the results. The report should contain:
    - [x] A description of the experiment. It should also contain descriptions of the code.
    - [x] Graphs and tables with the results
    - [x] Illustrations of the most important results
    - [x] Short comments about the results and graphs/tables/illustrations
- [x] Add a results section. The section should contain:
    - [x] List of primary results
    - [x] Recomendations for file format use
