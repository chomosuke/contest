#!/bin/bash
mvn package
java -cp target/java-1.0-SNAPSHOT.jar com.chomosuke.contest.Main
