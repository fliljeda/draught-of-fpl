

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
  data() {
    return {
      x: 0,
      y: 0
    }
  },
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
          <div> GW:  {{ team.gw_points}} {{ team.gw_projected_points != team.gw_points ? ('(' + team.gw_projected_points + ')') : '' }} </div>
          <div class="opponent"> 
            {{ opponent.gw_points }} {{ opponent.gw_projected_points != opponent.gw_points ? ('(' + opponent.gw_projected_points + ')') : '' }} - {{ opponent.team_name }}
          </div>
        </div>
        <div style="font-size:0.6em" v-for="player in team.players" :key="player.id" > 
          <div v-if="player.on_field">
            <div v-if="player.play_status.type == 'playing'" style="display: flex; flex-flow: row;" > 
              <img v-bind:title="player.team.name" style="width: 2em ; height: 2.5em;" v-bind:src="getShirtUrl(player)" /> 
              <div style="margin: auto 2px;" > 
                {{ player.display_name }}: {{ player.points }} {{ player.projected_points != player.points ? ('(' + player.projected_points + ')') : '' }} 
              </div>
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
                    {{ getSubbedWith(player).display_name }}: {{ getSubbedWith(player).points }} {{ getSubbedWith(player).projected_points != getSubbedWith(player).points ? ('(' + getSubbedWith(player).projected_points + ')') : '' }}
                  </div>
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
