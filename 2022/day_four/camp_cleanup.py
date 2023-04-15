from typing import Self, Optional
from dataclasses import dataclass


@dataclass
class ElfRange:
    """The area of the camp cleaned by an elf"""
    lower: int
    upper: int    

    @staticmethod
    def from_str(string_representation: str) -> Self:
        lower_bound, upper_bound = string_representation.split("-", 1)
        upper_bound = int(upper_bound)
        lower_bound = int(lower_bound)
        return ElfRange(lower_bound, upper_bound)
    
    @staticmethod
    def parse_line(line: str) -> tuple[Self, Self]:
        first, second = line.split(",", 1)
        return (ElfRange.from_str(first), ElfRange.from_str(second))
    
    @staticmethod
    def process_file(file_path: str) -> int:
        fully_contains_count = 0
        import os 
        with open(file_path, "r") as f:
            for line in f:
                first, second = ElfRange.parse_line(line)

                if first.fully_contains(second) or second.fully_contains(first):
                    fully_contains_count += 1
        return fully_contains_count


    def fully_contains(self, other: Self) -> bool:
        return other.lower >= self.lower and other.upper <= self.upper

if __name__ == "__main__":
    print("Hello World")

    import os, sys
    result = ElfRange.process_file("day_four/input.txt")
    print("Result is ", result)

