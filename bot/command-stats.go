package bot

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strconv"
	"strings"
	"time"

	"github.com/bwmarrin/discordgo"
)

const pURL string = "https://open.faceit.com/data/v4/players"

type FaceitData struct {
	Avatar string `json:"avatar"`
	Games  struct {
		Cs2 struct {
			Faceit_Elo  int64  `json:"faceit_elo"`
			Region      string `json:"region"`
			Skill_level int64  `json:"skill_level"`
		} `json:"cs2"`
	} `json:"games"`
	Nickname string `json:"nickname"`
	Country  string `json:"country"`
}

func getCurrentStats(message string, searchType string) *discordgo.MessageSend {
	var faceitURL string

	switch searchType {
	case "faceit-username":
		formattedUser := searchUser(message)
		if formattedUser == "" {
			return &discordgo.MessageSend{
				Content: "Sorry, unable to find user \"" + message + "\", make sure you entered a proper FACEIT username",
			}
		}
		faceitURL = fmt.Sprintf("%s?game=cs2&nickname=%s", pURL, formattedUser)
	case "steam-id":
		steamID := getSteamName(message)
		if steamID == "" {
			return &discordgo.MessageSend{
				Content: "Sorry, unable to find user \"" + message + "\", make sure you entered a proper Steam ID (either custom URL or ID64)",
			}
		}
		faceitURL = fmt.Sprintf("%s?game=cs2&game_player_id=%s", pURL, steamID)
	}

	// new HTTP client & timeout
	client := http.Client{Timeout: 5 * time.Second}

	req, err := http.NewRequest("GET", faceitURL, nil)
	if err != nil {
		return &discordgo.MessageSend{
			Content: "Sorry, there was an error trying to get stats (Possibly API key issue?).",
		}
	}

	var bearer = "Bearer " + string(FaceitToken)

	req.Header.Add("Authorization", bearer)
	resp, err := client.Do(req)

	// HTTP response body
	body, _ := io.ReadAll(resp.Body)
	defer resp.Body.Close()

	var data FaceitData

	json.Unmarshal([]byte(body), &data)

	// pull out info
	avatar := data.Avatar
	username := data.Nickname
	elo := strconv.FormatInt(data.Games.Cs2.Faceit_Elo, 10)
	faceitlvl := strconv.FormatInt(data.Games.Cs2.Skill_level, 10)
	region := data.Games.Cs2.Region
	upperregion := strings.ToUpper(region)
	usercountry := data.Country
	uppercountry := strings.ToUpper(usercountry)

	var regionstring string

	switch region {
	case "EU":
		regionstring = ":flag_eu:"
	}

	// embed response
	embed := &discordgo.MessageSend{
		Embeds: []*discordgo.MessageEmbed{{
			Thumbnail: &discordgo.MessageEmbedThumbnail{
				URL: avatar,
			},
			Type:        discordgo.EmbedTypeRich,
			Title:       username + "'s Stats",
			Description: "**[FACEIT](https://www.faceit.com/en/players/" + username + ")**",
			Fields: []*discordgo.MessageEmbedField{
				{
					Name:   "Elo",
					Value:  elo,
					Inline: true,
				},
				{
					Name:   "Level",
					Value:  faceitlvl,
					Inline: true,
				},
				{
					Name:   "Region",
					Value:  upperregion + regionstring,
					Inline: true,
				},
				{
					Name:   "Country",
					Value:  uppercountry + " :flag_" + usercountry + ":",
					Inline: true,
				},
			},
		},
		},
	}

	return embed
}
