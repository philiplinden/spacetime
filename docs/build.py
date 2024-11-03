# 1. generate docs pages from manim diagrams
# 2. generate markdown pages from jupyter notebooks
# 3. generate markdown pages for other static content like tables of content
# 4. update mkdocs.yml to include the new pages and point to the most recent
#    devlog page

import os
import pikepdf
from pathlib import Path

def generate_research_paper_links():
    # Define the directory containing the research papers
    directory = Path('./research-papers')
    # Define the output markdown file
    output_file = directory / 'links.md'

    # Create a markdown file and write the header
    with open(output_file, 'w') as md_file:
        md_file.write('# Research Papers\n\n')

        # Iterate through each file in the directory
        for file in os.listdir(directory):
            if file.endswith('.pdf'):  # Process only PDF files
                pdf_path = directory / file
                try:
                    # Open the PDF file and extract metadata
                    with pikepdf.open(pdf_path) as pdf:
                        docinfo = pdf.docinfo
                        title = docinfo.get('/Title', 'No Title')
                        author = docinfo.get('/Author', 'Unknown Author')
                        creation_date = docinfo.get('/CreationDate', 'Unknown Date')

                        # Create an intelligent label
                        label = f"{title} by {author} (Created on: {creation_date})"
                        md_file.write(f'- [{label}](./{pdf_path})\n')
                except Exception as e:
                    print(f"Error reading {file}: {e}")

    print(f'Markdown file "{output_file}" created with links to research papers.')

if __name__ == '__main__':
    generate_research_paper_links()
