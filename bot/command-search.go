package bot

import (
	"encoding/json"
	"encoding/xml"
	"fmt"
	"io"
	"log"
	"net/http"
	"strings"
	"time"
)

const sURL string = "https://open.faceit.com/data/v4/search/players?"

type SearchData struct {
	Items []struct {
		Nickname string `json:"nickname"`
	}
}

type SteamData struct {
	XMLName   xml.Name `xml:"profile"`
	SteamID64 string   `xml:"steamID64"`
}

func searchUser(message string) string {
	name := message

	searchURL := fmt.Sprintf("%snickname=%s", sURL, name)

	client := http.Client{Timeout: 5 * time.Second}

	req, err := http.NewRequest("GET", searchURL, nil)
	if err != nil {
		log.Fatal()
	}

	var bearer = "Bearer " + string(FaceitToken)

	req.Header.Add("Authorization", bearer)
	resp, err := client.Do(req)
	if err != nil {
		return ""
	}

	body, _ := io.ReadAll(resp.Body)
	defer resp.Body.Close()

	var data SearchData
	json.Unmarshal([]byte(body), &data)

	temp := data.Items[0].Nickname
	// pull info
out:
	for i := range 20 {
		playername := data.Items[i].Nickname
		if strings.EqualFold(playername, message) {
			temp = playername
			break out
		}
	}
	return temp
}

func getSteamName(message string) string {
	steamURL := "http://steamcommunity.com/id/" + message + "/?xml=1"

	client := http.Client{Timeout: 5 * time.Second}

	req, err := http.NewRequest("GET", steamURL, nil)
	if err != nil {
		log.Fatal()
	}

	resp, err := client.Do(req)
	if err != nil {
		return ""
	}

	body, _ := io.ReadAll(resp.Body)
	defer resp.Body.Close()

	var data SteamData
	xml.Unmarshal([]byte(body), &data)

	name := data.SteamID64
	return name
}
