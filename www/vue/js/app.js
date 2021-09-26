

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
    setInterval(httpGetAsync(dof_url, onResp), 5000);
  },
  data() {
    return {
      table: {},
      teams: []
    }
  },
  methods: {
    addNewTodo() {
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
  computed: {
  },
  methods: {
    getTable() {
      return this.$parent.table;
    },
    points() {
      let scoring = this.getTable().scoring;
      if (scoring === "H2H") {
        return this.team.h2h_info.points;
      } else {
        return this.team.total_points;
      }
    },
    opponent() {
      return this.getTable().entries.find(team => team.team_code == this.team.h2h_info.current_opponent);
    }
  },
  template: `
    <div>
      {{ team.team_name }} by {{ team.owner_name}} with {{ points() }} points
      is facing {{ opponent().team_name }} with {{ opponent().h2h_info.points }} points
    </div>
  `,
  props: {
    team: Object
  }
}

app.component('team-card', teamCard)
app.mount('#app');   
