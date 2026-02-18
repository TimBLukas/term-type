# «

from typing import List


class TxtConverter:
    def __init__(self, input_path: str, output_path: str):
        self.input_path = input_path
        self.output_path = output_path
        self.unicode_words: List[str] = []
        self.file_content: List[str] = []

    def load_content(self) -> None:
        with open(self.input_path) as f:
            content = f.readlines()
            self.file_content = content

    def parse_content(self) -> None:
        if self.file_content:
            for line in self.file_content:
                for word in line.split():
                    unicode_word = word.translate(
                        {ord(c): None for c in "«».,;:-'\"!?–—“”"}
                    )
                    if len(unicode_word) == 0:
                        continue
                    else:
                        self.unicode_words.append(unicode_word + "\n")

        else:
            print("No file content found, try loading first")

    def store_parsed_content(self) -> None:
        if self.unicode_words:
            with open(self.output_path, "w") as f:
                f.writelines(self.unicode_words)
        else:
            print("Unable to find unicode words, try parsing first")

    def run(self):
        print("Loading file content ...")
        self.load_content()
        print("Parsing file content ...")
        self.parse_content()
        print("Storing parsed content ...")
        self.store_parsed_content()
        print("Done!")


if __name__ == "__main__":
    input_path: str = "assets/books/illustration-of-today-english.txt"
    output_path: str = "assets/books/illustration-of-today-parsed-english.txt"
    converter = TxtConverter(input_path, output_path)
    converter.run()
