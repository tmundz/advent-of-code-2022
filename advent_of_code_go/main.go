package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func check(e error) {
    if e != nil {
        panic(e)
    }
}

func findBadgeValues(group []string) int {
    var lowerCaseConst int = 96
    var upperCaseConst int = 38
    points := 0
    for i := range group[0] {
        if strings.Contains(group[1], string(group[0][i])) {
            if strings.Contains(group[2], string(group[0][i])) {
                 if int(group[0][i]) < 97 {
                    points += int(group[0][i]) - upperCaseConst
                    return points
                }else {
                    points += int(group[0][i]) - lowerCaseConst
                    return points
                }
 
            }

        }
    }
    return points
}

func findBadge(ruck string) int {
    scanner := bufio.NewScanner(strings.NewReader(ruck));
    counter := 1
    var text []string
    var points int = 0
    for scanner.Scan() {
        text = append(text, scanner.Text())
        if (counter % 3) == 0 {
            points += findBadgeValues(text)
            text = nil
        }
        counter += 1 
    }
    return points
}

func findValues(ruck string) int{
    var point int = 0
    var lowerCaseConst int = 96
    var upperCaseConst int = 38
    var used string = ""
    
    compartment1 := ruck[0:(len(ruck)/2)]
    compartment2 := ruck[(len(ruck)/2):]

    for i := range compartment1 {
        if !(strings.Contains(used, string(compartment1[i]))) {
            if strings.Contains(compartment2,string(compartment1[i])) {

                if compartment1[i] < 97 {
                    used = used + string(compartment1[i])
                    point += int(compartment1[i]) - upperCaseConst

                }else {
                    used = used + string(compartment1[i])
                    point += int(compartment1[i]) - lowerCaseConst
                }
            }
        }
    }
    return point
}

func main() {
    //two large components
    //all items are meant to go into one of the two
    //1 item type per sack

    //a-z = 1-26
    //A-Z = 27 - 52
    var points int = 0
    file, err := os. ReadFile("ruck.txt")
    check(err);
    ruck := string(file);
    badgePoint := findBadge(ruck)
    fmt.Print(badgePoint, "\n")
    scanner := bufio.NewScanner(strings.NewReader(ruck));
    for scanner.Scan() {
        curRuck := scanner.Text()
        points += findValues(curRuck)
        
    }
    fmt.Print(points);

}
