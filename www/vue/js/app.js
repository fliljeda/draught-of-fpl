

function moveLoader() {
  document.getElementById("loader").classList.add("mover");
  document.getElementById("oskar").classList.remove("spinner");
  document.getElementById("loadertext").innerHTML = ""
}

function setLoaderTitle(title) {
  document.getElementById("leaguetitle").innerHTML = title;
}

function setLoaderFail() {
  document.getElementById("oskar").classList.remove("spinner");
  document.getElementById("loadertext").innerHTML = "Failed to fetch data.."
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
    that = this;
    onResp = function (resp) {
      if (resp) {
        that.table = JSON.parse(resp);
        that.updateTeams();
      }
      updateLoader(resp)
    }

    // Initial data fetch
    httpGetAsync("/table", function (resp) {
      onResp(resp)
    });

    // Continuously fetch data in intervals
    setInterval(function (resp) {
      httpGetAsync("/table", onResp);
    }, 30000);
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
      that.table.entries.sort((a, b) => {
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
      if (player.team_pos == "GK") {
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
      if (projected_points != player.points) {
        display_string += ' (' + projected_points + ')'
      }

      return display_string
    },
    pointSourceOrder(a, b) {
      const order = [
        'goals_scored',
        'assists',
        'penalties_saved',
        'defensive_contribution',
        'clean_sheets',
        'penalties_missed',
        'yellow_cards',
        'red_cards',
        'bonus',
      ];

      return order.indexOf(a.stat) - order.indexOf(b.stat);
    },
    getTeamAndPointsString(team) {
      // Returns points on the format:
      // <points> -- if team has no projected bonus points
      // <points> (<points + bonus>)  -- if team has projected bonus points
      let display_string = ""
      display_string += team.gw_points

      let projected_points = team.gw_projected_points
      if (projected_points != team.gw_points) {
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
        <div style=" margin-top: auto; margin-bottom: auto" class="row-container"> 
          <div style="font-size: 1.75rem;"> {{ team.team_name }} </div>
          <div style="font-size: 2rem; margin-left: auto;"> {{ points }}  </div> 
        </div>

        <div class="row-container"> 
          <div style="font-size: 1.4rem;"> GW: {{ getTeamAndPointsString(team) }} </div>
          <div style="font-size: 1.5rem;" v-if="opponent" class="opponent"> 
            {{ getTeamAndPointsString(opponent) }} - {{ opponent.team_name }}
          </div>
        </div>
        <div style="font-size:1rem" v-for="player in team.players" :key="player.id" > 
          <div v-if="player.on_field">

            <div class="row-container">
              <div v-if="player.play_status.type == 'playing'" class="row-container"> 
                <img v-bind:title="player.team.name" style="width: 1.2rem ; height: 1.5rem;" v-bind:src="getShirtUrl(player)" /> 
                <div style="margin: auto 2px;" > 
                  {{ player.display_name }}:
                </div>
                
                <span style="margin-right: 2px;" class="dot" :class="getPointsStatusColorClass(player)">
                    {{ getPlayerPointsString(player) }}
                </span>

              </div> 
              <div v-else class="row-container">
                <div v-if="player.play_status.type == 'subbed_off'" class="row-container" style="margin: auto 0px;" > 

                  <div class="row-container" style="opacity: 0.8;">
                    <img v-bind:title="player.team.name" style="width: 0.8rem ; height: 1rem;" v-bind:src="getShirtUrl(player)" /> 
                    <div style="margin: auto 1px; font-size:0.8rem"> {{ player.display_name }} </div> 
                  </div>

                  <div style="margin: auto 1px"> -> </div> 

                  <div class="row-container">
                    <img v-bind:title="getSubbedWith(player).team.name" style="width: 1.2rem ; height: 1.5rem;" v-bind:src="getShirtUrl(getSubbedWith(player))" />
                    <div style="margin: auto 2px"> 
                      {{ getSubbedWith(player).display_name }}:
                    </div>
                    <span class="dot" :class="getPointsStatusColorClass(getSubbedWith(player))">
                      {{ getPlayerPointsString(getSubbedWith(player)) }}
                    </span>
                  </div>
                </div> 
              </div>
              
              <!-- Point sources -->
              <div style="margin-left: 0.5rem" class="row-container" >
                <div style="margin: auto 0px;" class="row-container" v-for="point_source in player.point_sources.filter(p => p.points_total != 0).sort(pointSourceOrder)" >
                  <div v-if="point_source.stat == 'goals_scored'" v-for="x in point_source.amount" style="margin: auto; height: 1.25rem;"> 
                    <svg width="20" height="20" viewBox="-1 -1 16 16" fill="#222226" class="SvgWrapper eAoEla"><g clip-path="url(#clip0_1_842)" w="20" h="20" fill="#222226"><path d="M9.49999 5.83008L6.73999 6.83008V9.85008L8.81999 10.6601L10.89 8.37008L9.49999 5.83008Z" fill="#222226"></path><path d="M7 0C3.14 0 0 3.14 0 7C0 10.86 3.14 14 7 14C10.86 14 14 10.86 14 7C14 3.14 10.86 0 7 0ZM7 12.26C6.03 12.26 5.12 11.99 4.34 11.53L4.59 10.15L3.04 8.42L2.1 8.9C1.87 8.31 1.74 7.67 1.74 7C1.74 6.94 1.74 6.88 1.74 6.83L3.75 6.11L4.41 3.63L3.49 3.08C4.42 2.25 5.64 1.73 6.99 1.73C7.09 1.73 7.18 1.73 7.28 1.74L6.99 2.81L9.71 3.96L10.59 3.15C11.61 4.11 12.26 5.47 12.26 6.98C12.26 9.88 9.9 12.25 7 12.25V12.26Z" fill="#222226"></path></g></svg>
                  </div>
                  <div v-if="point_source.stat == 'assists'" v-for="x in point_source.amount" style="margin: auto; height: 1.25rem;"> 
                    <svg width="20" height="20" viewBox="0 0 16 16" fill="#222226" class="SvgWrapper eAoEla"><path d="m10.5 13.01 1.5 1.5-2.5.5-.5-.5 1.5-1.5zM7.92 1v1l2 1h2l1 6 1.5 4.5-1 1-10-10L5.92 1h2zM8 10.51l1.5 1.5-2.5.5-.5-.5 1.5-1.5zm-2.5-2.5L7 9.51l-2.5.5-.5-.5 1.5-1.5zm5.93.19H9.08l1.41 1.41h1.18l-.24-1.41zM3 5.51l1.5 1.5-2.5.5-.5-.5L3 5.51zm7.91-.01H6.43l1.41 1.41h3.31l-.24-1.41z" fill="#222226" fill-rule="evenodd" w="20" h="20"></path></svg>
                  </div>
                  <div v-if="point_source.stat == 'penalties_saved'" v-for="x in point_source.amount" style="margin: auto; height: 1.25rem;"> 
                    <svg class="SavedPenalty" width="20" height="20" viewBox="0 0 16 16" version="1.1" xmlns="http://www.w3.org/2000/svg"><g id="Page-1" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd"><g id="icPenaltySaved"><circle id="Ellipse_475" fill="#f1d359ff" fill-rule="nonzero" cx="8" cy="8" r="8"></circle><g id="ic-penalty-saved-2" transform="translate(2, 2) scale(1)"><polygon id="Rectangle_2688" points="0 0 12 0 12 12 0 12"></polygon><g id="Group_6464" transform="translate(0.244518, 0.014970)" fill="#333333"><path d="M1.76248221,6.78303029 L5.12348221,10.1500303 C5.20530852,10.2332914 5.20530852,10.3667691 5.12348221,10.4500303 L3.77848221,11.5730303 C3.67885792,11.6729345 3.54357029,11.7290836 3.40248221,11.7290836 C3.26139414,11.7290836 3.12610651,11.6729345 3.02648221,11.5730303 L0.156482214,8.69803029 C-0.052160738,8.4888263 -0.052160738,8.15023428 0.156482214,7.94103029 L1.15648221,6.77903029 C1.32067755,6.61207996 1.58898104,6.60939692 1.75648221,6.77303029 L1.76248221,6.78303029 Z" id="Path_4280"></path><path d="M10.4154822,8.70803029 C10.227686,8.49449721 10.0039804,8.31546506 9.75448221,8.17903029 C9.68972465,8.12359807 9.59423978,8.12359807 9.52948221,8.17903029 L8.78648221,8.84503029 C8.70805462,8.89307376 8.67590023,8.99080618 8.71048221,9.07603029 L8.97448221,10.0490303 C8.9983526,10.1016033 9.0466185,10.1390187 9.10348221,10.1490303 C9.19109359,10.1595219 9.27924504,10.1648644 9.36748221,10.1650303 L9.36748221,10.1580303 C9.58215664,10.1619094 9.79620034,10.1335949 10.0024822,10.0740303 C10.0628887,10.0481894 10.1150976,10.006353 10.1534822,9.95303029 C10.1595173,9.9460764 10.1639548,9.93788415 10.1664822,9.92903029 C10.3741899,9.62243698 10.4846614,9.26035519 10.4834916,8.89003029 C10.4811163,8.82360182 10.4572542,8.75973563 10.4154822,8.70803029 L10.4154822,8.70803029 Z" id="Path_4281" fill-rule="nonzero"></path><path d="M7.59948221,7.70803029 C7.59027231,7.62876253 7.52774998,7.56624019 7.44848221,7.55703029 C7.38198039,7.55040018 7.31498404,7.55040018 7.24848221,7.55703029 C7.02969156,7.55599037 6.81274562,7.5970709 6.60948221,7.67803029 C6.57512761,7.68730353 6.5435501,7.70480846 6.51748221,7.72903029 C6.50919732,7.73695293 6.50212172,7.74605013 6.49648221,7.75603029 C6.32347222,8.05583134 6.23242204,8.39588997 6.23248218,8.74203029 C6.23346982,8.78761067 6.24809957,8.83184826 6.27448221,8.86903029 C6.43570923,9.08981526 6.63976999,9.27585066 6.87448221,9.41603029 C6.90283404,9.43878613 6.93812782,9.45113895 6.97448221,9.451031 C7.01500032,9.45082222 7.05380397,9.43465403 7.08248221,9.40603029 L7.77448221,8.76103029 C7.81754559,8.71905185 7.83539219,8.65753804 7.82148221,8.59903029 L7.77648221,8.41103029 C7.71448221,8.17003029 7.65248221,7.93903029 7.59948221,7.70803029 L7.59948221,7.70803029 Z" id="Path_4282" fill-rule="nonzero"></path><path d="M9.42248221,5.84103029 L9.76248221,5.49903029 C9.99196908,5.26878039 9.99196908,4.89628019 9.76248221,4.66603029 L8.75148221,5.68003029 C8.61715465,5.65794361 8.48153733,5.6445823 8.34548221,5.64003029 C8.23190869,5.64524233 8.11872488,5.65692797 8.00648221,5.67503029 L10.1414822,3.53103029 C10.2989083,3.38437678 10.3638456,3.16355442 10.3108677,2.95502731 C10.2578897,2.7465002 10.0954267,2.58345069 9.88709185,2.52972163 C9.67875701,2.47599257 9.45770207,2.54013372 9.31048221,2.69703029 L7.08448221,4.93503029 L6.93348221,5.08603029 C6.82689485,5.17486641 6.67012615,5.16765697 6.57214217,5.06941309 C6.47415819,4.9711692 6.46736406,4.81438195 6.55648221,4.70803029 L6.70748221,4.55703029 L9.34848221,1.90203029 C9.55997834,1.66961391 9.55177363,1.31209657 9.32983827,1.08962707 C9.10790291,0.867157565 8.75040632,0.858093463 8.51748221,1.06903029 L5.87548221,3.72103029 L5.72448221,3.87203029 C5.61796462,3.96354953 5.45886358,3.95732261 5.35982464,3.85775827 C5.26078571,3.75819393 5.25540067,3.59906217 5.34748221,3.49303029 L5.49848221,3.34203029 L7.83748221,0.99203029 C8.05479759,0.760430835 8.04936314,0.398241073 7.82519713,0.173265791 C7.60103111,-0.0517094912 7.23886328,-0.0584490878 7.00648221,0.15803029 L4.66448221,2.50703029 L3.62548221,3.55303029 L3.91648221,4.68203029 C3.95416179,4.82439228 3.87036618,4.97058887 3.72848221,5.01003029 C3.65613993,5.0368877 3.57517864,5.0260179 3.51248221,4.98103029 L2.84148221,2.47303029 C2.72301714,2.03120249 2.26881001,1.76906521 1.82698221,1.88753029 C1.38515441,2.00599537 1.12301714,2.46020249 1.24148221,2.90203029 L1.88348221,5.29603029 L1.87448221,5.30503029 L2.13848221,6.48203029 L5.29848221,9.65303029 C5.69334645,11.1013291 7.05128573,12.0730159 8.54952231,11.9793438 C10.0477589,11.8856718 11.2740937,10.7524123 11.4854885,9.26620927 C11.6968832,7.7800062 10.8351783,6.34974788 9.42248221,5.84203029 L9.42248221,5.84103029 Z M10.4384822,10.4290303 C9.96395617,11.0428401 9.24353484,11.4169472 8.46848221,11.4520303 L8.38348221,11.1690303 L8.36548221,11.1060303 C8.35338584,11.0160902 8.27622847,10.9492205 8.18548221,10.9500303 C7.82122514,10.9453744 7.46695337,10.8303058 7.16948221,10.6200303 C7.13826389,10.6049286 7.10415407,10.5967423 7.06948221,10.5960303 C7.02741267,10.5922575 6.98559749,10.6055951 6.95348221,10.6330303 L6.68948221,10.8810303 C5.83106369,10.2005298 5.48116677,9.06116315 5.80968607,8.01615678 C6.13820537,6.97115042 7.07709935,6.23694173 8.17048221,6.17003029 L8.22348221,6.35203029 C8.24448221,6.41203029 8.26148221,6.47403029 8.27648221,6.53703029 C8.29748923,6.59521097 8.34330154,6.64102328 8.40148221,6.66203029 L8.42748221,6.66203029 C8.78125378,6.71403582 9.12472866,6.82099215 9.44548221,6.97903029 C9.4687546,6.98949191 9.49396669,6.99494317 9.51948221,6.99503029 C9.55449499,6.99415605 9.58878743,6.98489709 9.61948221,6.96803029 L9.96548221,6.72003029 C11.1196176,7.61358871 11.3313405,9.2733532 10.4384822,10.4280303 L10.4384822,10.4290303 Z" id="Path_4283" fill-rule="nonzero"></path></g></g></g></g></svg>
                  </div>
                  <div v-if="point_source.stat == 'defensive_contribution'" style="margin: auto; height: 1.25rem;">
                    <svg width="20" height="20"  fill="#000000" version="1.1" id="Capa_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 471.787 471.787" xml:space="preserve"><g stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <g> <g id="_x35_1_20_"> <path d="M360.852,35.142c-15.477-18.056-102.336-61.626-149.625-12.615c-47.29,49.01,2.952,83.636,21.012,91.97 c18.057,8.334,69.647,21.066,88.354-11.607c4.99,12.785,1.623,119.131-27.865,146.17c-14.942-14.246-36.51-23.19-60.488-23.19 c-19.689,0-37.746,6.031-51.85,16.073c-18.619-29.884-53.845-50.062-94.271-50.062c-19.383,0-37.563,4.659-53.308,12.782v10.448 c-0.013-0.003-0.056-0.013-0.056-0.013v256.662c0,0,74.807,3.87,80.791-82.544c-0.002-0.005-0.005-0.01-0.005-0.015 c18.198,26.427,76.18,46.541,111.909,45.355c56.121-1.861,130.693-4.321,193.865-64.881c5.838-5.809,10.52-12.669,13.701-20.259 c0-0.002,0-0.002,0-0.004C462.242,288.615,376.328,53.198,360.852,35.142z"></path> </g> </g> </g></svg>
                  </div>
                  <div v-if="point_source.stat == 'yellow_cards'" v-for="x in point_source.amount" style="margin: auto; height: 1.25rem;"> 
                    <svg width="20" height="20" viewBox="0 0 16 16" fill="#d9af00" style="min-width: 12px; display: inline-block;" shape-rendering="crispEdges" class="SvgWrapper fyGiev"><path fill="#d9af00" d="M3 1h10v14H3z" fill-rule="evenodd" shape-rendering="crispEdges" style="min-width: 12px; display: inline-block;"></path></svg>
                  </div>
                  <div v-if="point_source.stat == 'red_cards'" v-for="x in point_source.amount" style="margin: auto; height: 1.25rem;"> 
                    <svg width="20" height="20" viewBox="0 0 16 16" fill="#c7361f" style="min-width: 12px; display: inline-block;" shape-rendering="crispEdges" class="SvgWrapper fyGiev"><path fill="#c7361f" d="M3 1h10v14H3z" fill-rule="evenodd" shape-rendering="crispEdges" style="min-width: 12px; display: inline-block;"></path></svg>
                  </div>
                  <div v-if="point_source.stat == 'clean_sheets'" v-for="x in point_source.amount" style="margin: auto; height: 1.25rem;">
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><g stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M11.302 21.6149C11.5234 21.744 11.6341 21.8086 11.7903 21.8421C11.9116 21.8681 12.0884 21.8681 12.2097 21.8421C12.3659 21.8086 12.4766 21.744 12.698 21.6149C14.646 20.4784 20 16.9084 20 12V6.6C20 6.04207 20 5.7631 19.8926 5.55048C19.7974 5.36198 19.6487 5.21152 19.4613 5.11409C19.25 5.00419 18.9663 5.00084 18.3988 4.99413C15.4272 4.95899 13.7136 4.71361 12 3C10.2864 4.71361 8.57279 4.95899 5.6012 4.99413C5.03373 5.00084 4.74999 5.00419 4.53865 5.11409C4.35129 5.21152 4.20259 5.36198 4.10739 5.55048C4 5.7631 4 6.04207 4 6.6V12C4 16.9084 9.35396 20.4784 11.302 21.6149Z" stroke="#000000" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path> </g></svg>
                  </div>
                  <div v-if="point_source.stat == 'penalties_missed'" v-for="x in point_source.amount" style="margin: auto; height: 1.25rem;"> 
                    <svg width="20" height="20" viewBox="0 0 16 16" fill="#c7361f" style="min-width: 16px; display: inline-block;" class="SvgWrapper fyGiev"><title>Missed penalty</title><path d="M12.6667 0H12H1.33333H0V6H1.33333V1.33333H12V6H13.3333V0H12.6667Z" fill="#c7361f" style="min-width: 16px; display: inline-block;"></path><path d="M9.27328 5.44666L6.65995 8.06666L4.07328 5.47999L3.13328 6.42666L5.71328 9.00666L3.11328 11.6133L4.05995 12.5533L6.65995 9.95332L9.25995 12.5533L10.1999 11.6133L7.59995 9.00666L10.2199 6.39332L9.27328 5.44666Z" fill="#c7361f" style="min-width: 16px; display: inline-block;"></path></svg>
                  </div>
                  <div v-if="point_source.stat == 'bonus'" style="margin: auto; height: 1.25rem;"> 
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                      <g stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier">
                        <path d="M4.63495 14.4151L5.67396 12.2121C5.80734 11.9293 6.19266 11.9293 6.32604 12.2121L7.36505 14.4151L9.68859 14.7706C9.98671 14.8162 10.1055 15.1997 9.8897 15.4198L8.2087 17.1334L8.60542 19.5543C8.65637 19.8652 8.34456 20.1022 8.07781 19.9554L6 18.8118L3.92219 19.9554C3.65544 20.1022 3.34363 19.8652 3.39458 19.5543L3.7913 17.1334L2.1103 15.4198C1.89447 15.1997 2.01329 14.8162 2.31141 14.7706L4.63495 14.4151Z" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                        <path v-if="point_source.amount > 1" d="M16.6349 14.4151L17.674 12.2121C17.8073 11.9293 18.1927 11.9293 18.326 12.2121L19.3651 14.4151L21.6886 14.7706C21.9867 14.8162 22.1055 15.1997 21.8897 15.4198L20.2087 17.1334L20.6054 19.5543C20.6564 19.8652 20.3446 20.1022 20.0778 19.9554L18 18.8118L15.9222 19.9554C15.6554 20.1022 15.3436 19.8652 15.3946 19.5543L15.7913 17.1334L14.1103 15.4198C13.8945 15.1997 14.0133 14.8162 14.3114 14.7706L16.6349 14.4151Z" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                        <path v-if="point_source.amount > 2" d="M10.6349 5.41515L11.674 3.21211C11.8073 2.9293 12.1927 2.9293 12.326 3.21211L13.3651 5.41515L15.6886 5.7706C15.9867 5.8162 16.1055 6.19974 15.8897 6.41976L14.2087 8.13337L14.6054 10.5543C14.6564 10.8652 14.3446 11.1022 14.0778 10.9554L12 9.81178L9.92219 10.9554C9.65544 11.1022 9.34363 10.8652 9.39458 10.5543L9.7913 8.13337L8.1103 6.41976C7.89447 6.19974 8.01329 5.8162 8.31141 5.7706L10.6349 5.41515Z" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path> 
                      </g>
                    </svg>
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
