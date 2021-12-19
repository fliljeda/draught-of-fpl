

function moveLoader() {
    document.getElementById("loader").classList.add("mover");
    document.getElementById("oskar").classList.remove("spinner");
}

function setLoaderTitle(title) {
    document.getElementById("oskartext").innerHTML=title;
}

function setLoaderFail() {
    document.getElementById("oskar").classList.remove("spinner");
    document.getElementById("oskartext").innerHTML="Failed to fetch data.."
}

function updateLoader(resp) {
  if (resp) {
    setLoaderTitle(that.table.name);
    moveLoader();
  } else {
    setLoaderFail();
  }
}

const app = Vue.createApp({
  created() {
    that=this;
    onResp = function(resp) {
      if (resp) {
        that.table = JSON.parse(resp);
        that.updateTeams();
      }
      updateLoader(resp)
    }

    // Initial data fetch
    httpGetAsync(dof_url, function(resp) {
      onResp(resp)
    });

    // Continuously fetch data in intervals
    setInterval(function(resp) {
      httpGetAsync(dof_url, onResp);
    }, 5000);
  },
  data() {
    return {
      table: {},
      teams: []
    }
  },
  methods: {
    doSomething() {
    },
    updateTeams() {

      // Sort by points in reverse order to create elements with most points first
      that.table.entries.sort((a,b) => {
        if (that.table.scoring === "H2H") {
          return b.h2h_info.points - a.h2h_info.points;
        } else {
          return b.total_points - a.total_points;
        }
      });
      that.teams = that.table.entries;
    }
  }
})

teamCard = {
  props: {
    team: Object
  },
  computed: {
    getTable() {
      return this.$parent.table;
    },
    points() {
      let scoring = this.getTable.scoring;
      if (scoring === "H2H") {
        return this.team.h2h_info.points;
      } else {
        return this.team.total_points;
      }
    },
    opponent() {
      return this.getTable.entries.find(team => team.team_code == this.team.h2h_info.current_opponent);
    }
  },
  methods: {
    getShirtUrl(player) {
      if (player.team_pos.name == "GK") {
        return player.team.gk_shirt_url;
      } else {
        return player.team.shirt_url;
      }
    },
    getSubbedWith(player) {
      let subbed_with_id = player.play_status.subbed_with;
      return this.team.players.find(p => p.id == subbed_with_id);
    },
    getPlayerPointsString(player) {
      // Returns points on the format:
      // <points> -- if player has no projected bonus points
      // <points> (<points + bonus>)  -- if player has projected bonus points

      let display_string = ""
      display_string += player.points

      let projected_points = player.projected_points  
      if (projected_points != player.points ) {
        display_string += ' (' + projected_points + ')'
      }

      return display_string
    },
    getTeamAndPointsString(team) {
      // Returns points on the format:
      // <points> -- if team has no projected bonus points
      // <points> (<points + bonus>)  -- if team has projected bonus points
      let display_string = ""
      display_string += team.gw_points

      let projected_points = team.gw_projected_points  
      if (projected_points != team.gw_points ) {
        display_string += ' (' + projected_points + ')'
      }

      return display_string
    },
    getPointsStatusColorClass(player) {
      // Returns the class for what the status of the points are.
      // If the player has upcoming matches it is marked as yellow with class: dot-color-yellow
      // If the player has played and has no upcoming matches it is marked as green with class: dot-color-green
      // If the player has no upcoming matches and has not played: dot-color-red
      return {
          "dot-color-yellow": !player.fixtures_finished,
          "dot-color-green": player.has_played && player.fixtures_finished,
          "dot-color-red": !player.has_played && player.fixtures_finished
      }
    }
  },
  template: `
    <div class="team-card" >
      <div> 
        <div style="font-size: 1.3em; display: flex; flex-flow: row;"> 
          <div> {{ team.team_name }} </div>
          <div style="font-size: 1.2em; margin-left: auto; margin-top: auto; margin-bottom: auto"> {{ points }}  </div> 
        </div>

        <div style="display: flex; flex-flow: row;"> 
          <div> GW: {{ getTeamAndPointsString(team) }} </div>
          <div class="opponent"> 
            {{ getTeamAndPointsString(opponent) }} - {{ opponent.team_name }}
          </div>
        </div>
        <div style="font-size:0.6em" v-for="player in team.players" :key="player.id" > 
          <div v-if="player.on_field">

            <div v-if="player.play_status.type == 'playing'" style="display: flex; flex-flow: row;" > 
              <img v-bind:title="player.team.name" style="width: 2em ; height: 2.5em;" v-bind:src="getShirtUrl(player)" /> 
              <div style="margin: auto 2px;" > 
                {{ player.display_name }}:
              </div>
              <span class="dot" :class="getPointsStatusColorClass(player)">
                  {{ getPlayerPointsString(player) }}
              </span>
            </div> 
            <div v-else style="display: flex; flex-flow: row;" >
             
              <div v-if="player.play_status.type == 'subbed_off'"  style="margin: auto 0px; display: flex; flex-flow: row;" > 

                <div style="display: flex; flex-flow: row; opacity: 0.8;">
                  <img v-bind:title="player.team.name" style="width: 2em ; height: 2.5em;" v-bind:src="getShirtUrl(player)" /> 
                  <div style="margin: auto 2px"> {{ player.display_name }} </div> 
                </div>

                <div style="margin: auto 2px"> -> </div> 

                <div style="display: flex; flex-flow: row;">
                  <img v-bind:title="getSubbedWith(player).team.name" style="width: 2em ; height: 2.5em;" v-bind:src="getShirtUrl(getSubbedWith(player))" />
                  <div style="margin: auto 2px"> 
                    {{ getSubbedWith(player).display_name }}:
                  </div>
                  <span class="dot" :class="getPointsStatusColorClass(getSubbedWith(player))">
                    {{ getPlayerPointsString(getSubbedWith(player)) }}
                  </span>
                </div>

              </div> 
            </div>
          </div>
        </div> 
      </div>
    </div>
  `
}

app.component('team-card', teamCard)
app.mount('#app');   
