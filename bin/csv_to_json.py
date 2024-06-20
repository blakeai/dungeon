#!/usr/bin/env python3

import json
import re

CLASS = "class"
SUBCLASSES = "subclasses"
WEAPON_PROF = "weaponproficiency"
ARMOR_PROF = "armorproficiency"
SAVING_THROWS = "savingthrows"
SKILLS = "skills"
LEVEL = "level"
LEVELS = "levels"


def level(n):
    return f"{LEVEL}{n}"


def levels():
    return [level(n) for n in range(1, 13)]


headers = [CLASS, WEAPON_PROF, ARMOR_PROF, SAVING_THROWS, SKILLS] + levels()


def index_of(header):
    return headers.index(header)


def read_lines(file_name):
    with open(file_name, "r") as file:
        return file.readlines()


def read_csv(file_name):
    lines = read_lines(file_name)
    lines = [line.replace("\n", "") for line in lines]

    def split_not_in_quotes(s):
        # Regular expression to match commas not within quotes
        pattern = re.compile(r',(?=(?:[^"]*"[^"]*")*[^"]*$)')
        split = pattern.split(s)
        removed_quotes = [x.replace("\"", "") for x in split]
        return removed_quotes

    return [split_not_in_quotes(line) for line in lines]


def to_json_row(row):
    proficiencies = [WEAPON_PROF, ARMOR_PROF, SAVING_THROWS, SKILLS]
    proficiency_indices = [(index_of(header), header) for header in proficiencies]
    output = {header: row[index] for index, header in proficiency_indices if row[index] != ""}

    levels = [level(n) for n in range(1, 13)]
    level_indices = [(index_of(header), header) for header in levels]
    level_entry = {header.replace(LEVEL, ""): row[index] for index, header in level_indices if row[index] != ""}
    output.update({LEVELS: level_entry})
    return output


def create_classes_and_subclasses(data):
    current_class = None
    output = {}

    for row in data:
        key = row[index_of(CLASS)]
        if not key.startswith("- "):
            current_class = key
            output[current_class] = {CLASS: to_json_row(row), SUBCLASSES: {}}
        else:
            output[current_class][SUBCLASSES][key[2:]] = to_json_row(row)

    return output


def convert_to_json(csv):
    assert csv[0] == headers
    return json.dumps(create_classes_and_subclasses(csv[1:]), indent=4)


if __name__ == '__main__':
    csv = read_csv("bg3.csv")
    print(convert_to_json(csv))
