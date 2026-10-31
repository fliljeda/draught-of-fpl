#!/usr/bin/env python3
"""
Saves the FPL API responses for a league to tests/snapshots/<name>/api/...

    python db/snapshot.py 26952 double-gw
    python db/snapshot.py 26952 season-so-far --complete

Member names are replaced before saving. --complete also fetches every earlier
gameweek, instead of only the current one.
"""

import argparse
import json
import sys
import time
from pathlib import Path

import requests

HOST = "https://draft.premierleague.com"
SNAPSHOTS = Path(__file__).resolve().parent.parent / "tests" / "snapshots"


def fetch(path):
    print(path)
    time.sleep(0.2)  # Be polite to FPL
    response = requests.get(HOST + path, timeout=30)
    response.raise_for_status()
    return response.json()


def main():
    parser = argparse.ArgumentParser(description="Save FPL API responses for tests")
    parser.add_argument("league_id", type=int)
    parser.add_argument("name", help="Folder name under tests/snapshots")
    parser.add_argument(
        "--complete", action="store_true", help="Fetch all gameweeks so far"
    )
    args = parser.parse_args()

    folder = SNAPSHOTS / args.name
    if folder.exists():
        sys.exit(f"{folder} already exists")

    responses = {}
    responses["api/game"] = fetch("/api/game")
    responses["api/bootstrap-static"] = fetch("/api/bootstrap-static")
    details = fetch(f"/api/league/{args.league_id}/details")
    responses[f"api/league/{args.league_id}/details"] = details

    # Replace names. Numbering by entry_id keeps a member's number between snapshots.
    details["league"]["name"] = "Test League"
    entries = sorted(details["league_entries"], key=lambda e: e["entry_id"])
    numbers = {e["entry_id"]: f"{i:02d}" for i, e in enumerate(entries, start=1)}

    for entry in entries:
        number = numbers[entry["entry_id"]]
        entry["entry_name"] = f"Team {number}"
        entry["player_first_name"] = "Manager"
        entry["player_last_name"] = number
        entry["short_name"] = number

    for entry_id, number in numbers.items():
        public = fetch(f"/api/entry/{entry_id}/public")
        public["entry"]["name"] = f"Team {number}"
        public["entry"]["player_first_name"] = "Manager"
        public["entry"]["player_last_name"] = number
        public["entry"]["favourite_team"] = None
        public["entry"]["region_name"] = None
        public["entry"]["region_code_short"] = None
        public["entry"]["region_code_long"] = None
        responses[f"api/entry/{entry_id}/public"] = public

    # current_event is None in preseason
    current_gw = responses["api/game"]["current_event"] or 0
    first_gw = 1 if args.complete else current_gw
    for gw in range(max(first_gw, 1), current_gw + 1):
        responses[f"api/event/{gw}/live"] = fetch(f"/api/event/{gw}/live")
        for entry_id in numbers:
            path = f"api/entry/{entry_id}/event/{gw}"
            responses[path] = fetch(f"/{path}")

    for path, data in responses.items():
        file_path = folder / path
        file_path.parent.mkdir(parents=True, exist_ok=True)
        with open(file_path, "w", encoding="utf-8", newline="\n") as f:
            json.dump(data, f, indent=2, ensure_ascii=False)

    print(f"Saved {len(responses)} responses to {folder}")


if __name__ == "__main__":
    main()
