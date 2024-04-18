package main

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"sync"
)

func downloadPart(url string, start int64, end int64, wg *sync.WaitGroup, file *os.File) {
	defer wg.Done()

	client := http.Client{}
	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		fmt.Printf("Error creating request: %s\n", err)
		return
	}

	// Specify the range of bytes to download for this part
	req.Header.Set("Range", fmt.Sprintf("bytes=%d-%d", start, end))

	res, err := client.Do(req)
	if err != nil {
		fmt.Printf("Error downloading part: %s\n", err)
		return
	}
	defer res.Body.Close()

	// Write the downloaded part to the file
	_, err = io.Copy(file, res.Body)
	if err != nil {
		fmt.Printf("Error writing part to file: %s\n", err)
		return
	}
}

func mergeFile(parts []*os.File, outputFilePath string) error {
	outputFile, err := os.Create(outputFilePath)
	if err != nil {
		return fmt.Errorf("Error creating output file: %s", err)
	}
	defer outputFile.Close()

	for _, part := range parts {
		_, err := part.Seek(0, 0)
		if err != nil {
			return fmt.Errorf("Error seeking to the beginning of part: %s", err)
		}

		_, err = io.Copy(outputFile, part)
		if err != nil {
			return fmt.Errorf("Error writing part to output file: %s", err)
		}

		part.Close()
	}

	return nil
}

func main() {
	url := "https://proof.ovh.net/files/10Mb.dat"
	numParts := 4

	// Get the file size
	resp, err := http.Head(url)
	if err != nil {
		fmt.Printf("Error getting file size: %s\n", err)
		return
	}
	fileSize := resp.ContentLength

	// Calculate the size of each part
	partSize := fileSize / int64(numParts)

	// Create a slice to hold the file parts
	parts := make([]*os.File, numParts)

	// Create a wait group to wait for all parts to finish downloading
	var wg sync.WaitGroup
	wg.Add(numParts)

	// Download each part concurrently
	for i := 0; i < numParts; i++ {
		start := int64(i) * partSize
		end := start + partSize - 1

		if i == numParts-1 {
			// Adjust the end for the last part to handle any remaining bytes
			end = fileSize - 1
		}

		partFile, err := os.Create(fmt.Sprintf("part%d.tmp", i))
		if err != nil {
			fmt.Printf("Error creating part file: %s\n", err)
			return
		}

		parts[i] = partFile

		go downloadPart(url, start, end, &wg, partFile)
	}

	// Wait for all parts to finish downloading
	wg.Wait()

	// Merge the file parts
	err = mergeFile(parts, "output.bin")
	if err != nil {
		fmt.Printf("Error merging file parts: %s\n", err)
		return
	}

	fmt.Println("File download and merge completed successfully!")
}
