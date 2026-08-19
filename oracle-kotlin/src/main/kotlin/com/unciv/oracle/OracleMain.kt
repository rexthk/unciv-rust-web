package com.unciv.oracle

import com.badlogic.gdx.Gdx
import com.badlogic.gdx.backends.headless.HeadlessFiles
import com.badlogic.gdx.utils.JsonReader
import com.unciv.UncivGame
import com.unciv.logic.GameInfo
import com.unciv.logic.GameStarter
import com.unciv.logic.civilization.PlayerType
import com.unciv.logic.files.UncivFiles
import com.unciv.logic.map.MapParameters
import com.unciv.logic.map.MapSize
import com.unciv.models.metadata.GameParameters
import com.unciv.models.metadata.GameSettings
import com.unciv.models.metadata.GameSetupInfo
import com.unciv.models.metadata.Player
import com.unciv.models.ruleset.RulesetCache
import com.unciv.utils.Concurrency
import java.io.File
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.buildJsonArray
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

object OracleMain {
    private const val marker = "@@UNCIV_ORACLE_JSON@@"

    @JvmStatic
    fun main(args: Array<String>) {
        require(args.size == 1) { "Usage: OracleMain <scenario.json>" }
        val scenarioFile = File(args[0])
        require(scenarioFile.isFile) { "Scenario does not exist: ${scenarioFile.absolutePath}" }

        val scenario = JsonReader().parse(scenarioFile.readText())
        require(scenario.getInt("schemaVersion") == 1) { "Unsupported scenario schema" }
        val seed = scenario.getLong("seed")

        Gdx.files = HeadlessFiles()
        UncivGame.Current = UncivGame(true).apply {
            settings = GameSettings().apply {
                showTutorials = false
                turnsBetweenAutosaves = 10000
            }
            files = UncivFiles(Gdx.files)
        }
        RulesetCache.loadRulesets(consoleMode = true, noMods = true)

        var game: GameInfo? = null
        var selectedUnit: String? = null
        val instructionsApplied = mutableListOf<String>()
        var instruction = scenario.get("instructions").child
        while (instruction != null) {
            val kind = instruction.getString("type")
            when (kind) {
                "new_game" -> game = createGame(seed)
                "select_first_unit" -> {
                    val activeGame = requireNotNull(game) { "select_first_unit requires new_game" }
                    val unit = activeGame.getCurrentPlayerCivilization().units.getCivUnits()
                        .sortedWith(compareBy({ it.name }, { it.id }))
                        .first()
                    val position = unit.getTile().position
                    selectedUnit = "${unit.name}@${position.x},${position.y}"
                }
                "noop" -> Unit
                else -> error("Unsupported oracle instruction: $kind")
            }
            instructionsApplied.add(kind)
            instruction = instruction.next
        }

        val activeGame = requireNotNull(game) { "Scenario did not create a game" }
        val normalized = buildJsonObject {
            put("schemaVersion", 1)
            put("compatibilityVersion", "4.21.10")
            put("seed", seed)
            put("turn", activeGame.turns)
            put("instructionsApplied", buildJsonArray {
                instructionsApplied.forEach { add(JsonPrimitive(it)) }
            })
            put("metadata", buildJsonObject {
                put("rulesets", RulesetCache.size)
                put("civilizations", buildJsonArray {
                    activeGame.civilizations.map { it.civName }.sorted().forEach { add(JsonPrimitive(it)) }
                })
                put("tileCount", activeGame.tileMap.values.size)
                put("unitCount", activeGame.civilizations.sumOf { it.units.getCivUnits().count() })
                selectedUnit?.let { put("selectedUnit", it) }
                put("tiles", buildJsonArray {
                    activeGame.tileMap.values
                        .sortedWith(compareBy({ it.position.x }, { it.position.y }))
                        .forEach { tile ->
                            val features = tile.terrainFeatures.sorted().joinToString("+")
                            add(JsonPrimitive(
                                "${tile.position.x},${tile.position.y}:${tile.baseTerrain}:$features"
                            ))
                        }
                })
                put("units", buildJsonArray {
                    activeGame.civilizations
                        .flatMap { civilization ->
                            civilization.units.getCivUnits().map { unit ->
                                val position = unit.getTile().position
                                "${civilization.civName}:${unit.name}@${position.x},${position.y}"
                            }
                        }
                        .sorted()
                        .forEach { add(JsonPrimitive(it)) }
                })
            })
        }
        println(marker + normalized)
        Concurrency.stopThreadPools()
    }

    private fun createGame(seed: Long): GameInfo {
        val gameParameters = GameParameters().apply {
            numberOfCityStates = 0
            noBarbarians = true
            players = arrayListOf(
                Player("Rome", PlayerType.Human),
                Player("Greece"),
            )
        }
        val mapParameters = MapParameters().apply {
            mapSize = MapSize.Tiny
            this.seed = seed
            noRuins = true
            noNaturalWonders = true
        }
        return GameStarter.startNewGame(GameSetupInfo(gameParameters, mapParameters)).also {
            UncivGame.Current.gameInfo = it
        }
    }
}
