package api

import (
	"fmt"
	"html"
	"net/http"
	"net/url"
	"strings"

	"github.com/Achno/gowall/config"
	"github.com/PuerkitoBio/goquery"
)

func GetWallpaperOfTheDay() (string, error) {
	req, err := http.NewRequest(http.MethodGet, config.WallOfTheDayUrl+"?sort=top&t=day", nil)
	if err != nil {
		return "", err
	}
	req.Header.Set("User-Agent", "Mozilla/5.0 (compatible; gowall/1.0)")
	req.Header.Set("Accept", "text/html")

	response, err := (&http.Client{}).Do(req)
	if err != nil {
		return "", err
	}
	defer response.Body.Close()

	if response.StatusCode != http.StatusOK {
		return "", fmt.Errorf("request failed with status code: %d %s", response.StatusCode, http.StatusText(response.StatusCode))
	}
	doc, err := goquery.NewDocumentFromReader(response.Body)
	if err != nil {
		return "", err
	}
	post := firstPostSelection(doc)
	if post == nil {
		return "", fmt.Errorf("could not find a post element on the page")
	}
	if imageURL := firstPostImageURL(post); imageURL != "" {
		return imageURL, nil
	}
	return "", fmt.Errorf("there wasn't a top wallpaper today :( check later")
}

func firstPostSelection(doc *goquery.Document) *goquery.Selection {
	for _, selector := range []string{"shreddit-post", "article", "[data-testid='post-container']", ".thing"} {
		if post := doc.Find(selector).First(); post.Length() > 0 {
			return post
		}
	}
	return nil
}

func firstPostImageURL(post *goquery.Selection) string {
	imageAttrs := []string{"src", "data-src"}

	if u, ok := post.Attr("data-url"); ok {
		if clean := cleanImageURL(u); isRedditImageURL(clean) {
			return clean
		}
	}

	var found string
	post.Find("img, source").EachWithBreak(func(_ int, s *goquery.Selection) bool {
		if srcset, ok := s.Attr("srcset"); ok {
			if u := bestFromSrcset(srcset); isRedditImageURL(u) {
				found = u
				return false
			}
		}
		for _, attr := range imageAttrs {
			if u, ok := s.Attr(attr); ok {
				if clean := cleanImageURL(u); isRedditImageURL(clean) {
					found = clean
					return false
				}
			}
		}
		return true
	})
	if found != "" {
		return found
	}

	post.Find("a[href]").EachWithBreak(func(_ int, s *goquery.Selection) bool {
		if u, ok := s.Attr("href"); ok {
			if clean := cleanImageURL(u); isRedditImageURL(clean) {
				found = clean
				return false
			}
		}
		return true
	})
	return found
}

func bestFromSrcset(srcset string) string {
	best, bestW := "", 0
	for _, candidate := range strings.Split(srcset, ",") {
		fields := strings.Fields(strings.TrimSpace(candidate))
		if len(fields) == 0 {
			continue
		}
		w := 0
		if len(fields) > 1 {
			fmt.Sscanf(fields[1], "%dw", &w)
		}
		if clean := cleanImageURL(fields[0]); w > bestW && isRedditImageURL(clean) {
			best, bestW = clean, w
		}
	}
	return best
}

func cleanImageURL(imageURL string) string {
	imageURL = strings.TrimSpace(html.UnescapeString(imageURL))
	if imageURL == "" || strings.HasPrefix(imageURL, "data:") || strings.HasPrefix(imageURL, "blob:") {
		return ""
	}
	if strings.HasPrefix(imageURL, "//") {
		imageURL = "https:" + imageURL
	}
	parsed, err := url.Parse(imageURL)
	if err != nil {
		return imageURL
	}
	if strings.EqualFold(parsed.Hostname(), "preview.redd.it") {
		parsed.Scheme, parsed.Host, parsed.RawQuery, parsed.ForceQuery = "https", "i.redd.it", "", false
		return parsed.String()
	}
	return imageURL
}

func isRedditImageURL(imageURL string) bool {
	parsed, err := url.Parse(imageURL)
	if err != nil {
		return false
	}
	host := strings.ToLower(parsed.Hostname())
	return host == "i.redd.it" || host == "preview.redd.it" || host == "external-preview.redd.it"
}
