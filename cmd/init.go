/*
Copyright © 2025 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"
	"io"
	"os"
	"path/filepath"

	"github.com/spf13/cobra"
)

func copyTree(srcPath, destPath string) error {
	return filepath.Walk(srcPath, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		relPath, err := filepath.Rel(srcPath, path)
		if err != nil {
			return err
		}

		destFullPath := filepath.Join(destPath, relPath)

		if info.IsDir() {
			return os.MkdirAll(destFullPath, info.Mode())
		}

		return copyFile(path, destFullPath)
	})
}

func copyFile(src, dest string) error {
	srcFile, err := os.Open(src)
	if err != nil {
		return err
	}
	defer srcFile.Close()

	destFile, err := os.Create(dest)
	if err != nil {
		return err
	}
	defer destFile.Close()

	_, err = io.Copy(destFile, srcFile)
	if err != nil {
		return err
	}

	srcInfo, err := os.Stat(src)
	if err != nil {
		return err
	}

	return os.Chmod(dest, srcInfo.Mode())
}

// initCmd represents the init command
var initCmd = &cobra.Command{
	Use:   "init [project-path]",
	Short: "This command initializes the pent-hive project structure",
	Long: `This command initializes the pent-hive project structure, setting up necessary directories and files.
It supports an optional flag to include an example configuration file's.`,
	Args: cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {

		projectPath, err := filepath.Abs(args[0])
		projectName := filepath.Base(projectPath)

		fmt.Printf("%s %s %s %s\n", BasicTextStyle.Render("Initializing project"), NamesStyle.Render(projectName), BasicTextStyle.Render("at"), NamesStyle.Render(projectPath))

		if err != nil {
			println("Error resolving path:", err.Error())
			return
		}

		err = os.MkdirAll(projectPath, os.ModePerm)
		if err != nil {
			println("Error creating project directory:", err.Error())
			return
		}

		err = os.Chdir(projectPath)
		if err != nil {
			println("Error changing directory:", err.Error())
			return
		}

		fmt.Println(BasicTextStyle.Render("Creating project directories"))
		os.Mkdir("bees", os.ModePerm)
		os.Mkdir("hives", os.ModePerm)
		os.Mkdir("queens", os.ModePerm)

		withExampleConfig, err := cmd.Flags().GetBool("with-example-config")
		if err != nil {
			println("Error reading flag:", err.Error())
			return
		}

		if withExampleConfig {
			fmt.Println(BasicTextStyle.Render("Copying example configuration files"))
			copyTree("TODO_PATH", projectPath)
		}
	},
}

func init() {
	rootCmd.AddCommand(initCmd)

	initCmd.Flags().BoolP("with-example-config", "e", false, "Generate an example with the file structure of the project")
}
