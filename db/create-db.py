#!/usr/bin/env python3

import argparse
import json
import os
import shutil
import sys
from pathlib import Path
from urllib.parse import urlparse, urljoin

import requests


def check_prerequisites():
    """Check if required commands are available."""
    required_commands = ["jq", "curl"]
    missing = []
    
    for cmd in required_commands:
        if shutil.which(cmd) is None:
            missing.append(cmd)
    
    if missing:
        print(f"Missing required commands: {', '.join(missing)}")
        sys.exit(1)


def path_curl(url, base_path=""):
    """
    Fetch JSON from URL and save to local file structure.
    
    Args:
        url: Full URL to fetch from
        base_path: Base path for storing files (default: current directory)
    """
    api_prefix = "https://draft.premierleague.com/api/"
    
    # Extract relative path from URL
    if url.startswith(api_prefix):
        rel_path = url[len(api_prefix):]
    else:
        rel_path = url
    
    # Construct full file path
    file_path = os.path.join("api", base_path, rel_path)
    dir_path = os.path.dirname(file_path)
    
    # Create directory structure
    if dir_path:
        Path(dir_path).mkdir(parents=True, exist_ok=True)
    
    # Fetch data from URL
    try:
        response = requests.get(url)
        response.raise_for_status()
        data = response.json()
    except requests.exceptions.RequestException as e:
        print(f"Error fetching {url}: {e}")
        return None
    
    # Write to file if it doesn't exist
    if not os.path.exists(file_path):
        with open(file_path, 'w') as f:
            json.dump(data, f, indent=2)
        print(f"Created: {file_path}")
    else:
        print(f"Skipped (exists): {file_path}")
    
    return data


def main():
    parser = argparse.ArgumentParser(
        description="Fetch FPL draft data and create local database"
    )
    parser.add_argument(
        "league_code",
        type=str,
        help="League code for the draft league"
    )
    parser.add_argument(
        "-b", "--base-path",
        type=str,
        default=".",
        help="Base path for storing fetched data (default: current directory)"
    )
    parser.add_argument(
        "--skip-prerequisites",
        action="store_true",
        help="Skip checking for jq and curl prerequisites"
    )
    parser.add_argument(
        "--all-gws",
        action="store_true",
        help="Fetch all gameweeks up to current"
    )
    
    args = parser.parse_args()
    
    # Check prerequisites
    if not args.skip_prerequisites:
        check_prerequisites()
    
    league_code = args.league_code
    base_path = args.base_path
    api_prefix = "https://draft.premierleague.com/api"
    
    print(f"Fetching data for league: {league_code}")
    print(f"Saving to: {base_path}")
    
    # Fetch general things
    print("\nFetching bootstrap-static...")
    path_curl(f"{api_prefix}/bootstrap-static", base_path)
    
    # Get gameweek range
    print("Fetching game data...")
    game_data = path_curl(f"{api_prefix}/game", base_path)
    if not game_data:
        print("Failed to fetch game data")
        sys.exit(1)
    
    current_gw = game_data.get("current_event")
    print(f"Current gameweek: {current_gw}")
    
    # Get teams
    print(f"Fetching league details for {league_code}...")
    league_data = path_curl(f"{api_prefix}/league/{league_code}/details", base_path)
    if not league_data:
        print("Failed to fetch league details")
        sys.exit(1)
    
    teams = [entry.get("entry_id") for entry in league_data.get("league_entries", [])]
    print(f"Found {len(teams)} teams: {teams}")
    
    # Fetch team specific data
    print("\nFetching team-specific data...")
    for team_id in teams:
        path_curl(f"{api_prefix}/entry/{team_id}/public", base_path)
    
    # Fetch gameweek specific data
    if current_gw is not None:
        print(f"\nFetching gameweek data (GW 1 to {current_gw})...")
        if args.all_gws:
            for gw in range(1, current_gw + 1):
                # Fetch GW specific
                path_curl(f"{api_prefix}/event/{gw}/live", base_path)
                
                # Fetch GW + Team specific
                for team_id in teams:
                    path_curl(f"{api_prefix}/entry/{team_id}/event/{gw}", base_path)
        else:
            # Fetch GW specific
            path_curl(f"{api_prefix}/event/{current_gw}/live", base_path)
            
            # Fetch GW + Team specific
            for team_id in teams:
                path_curl(f"{api_prefix}/entry/{team_id}/event/{current_gw}", base_path)
    else:
        print("Current GW is null - preseason!")
        sys.exit(0)
    
    print("\nDatabase creation completed!")


if __name__ == "__main__":
    main()
