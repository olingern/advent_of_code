interface RockPaperScissorsPlayer {
  choice: RPS | null;
  score: number;
}

interface RockPaperScissorsGameState {
  playerOne: RockPaperScissorsPlayer;
  playerTwo: RockPaperScissorsPlayer;
}

enum RPS {
  ROCK = 1,
  PAPER = 2,
  SCISSORS = 3,
}

const LOOKUP_VALUES: Record<string, number> = {
  A: RPS.ROCK, // Rock
  B: RPS.PAPER, // Paper
  C: RPS.SCISSORS, // Scissors
  X: RPS.ROCK, // Rock
  Y: RPS.PAPER, // Paper
  Z: RPS.SCISSORS, // Scissors
};

const RESULT_VALUES: Record<string, number> = {
  LOSE: 0,
  DRAW: 3,
  WIN: 6,
};

function getPlayerChoiceValue(choiceStr: string): number {
  return LOOKUP_VALUES[choiceStr];
}

function getInitialPlayerState(): RockPaperScissorsPlayer {
  return {
    choice: null,
    score: 0,
  };
}

/**
 * Assumption: draws have been filtered out before this function is called
 *
 * @param playerOneChoice
 * @param playerTwoChoice
 * @returns boolean
 */
function playerOneWins(playerOneChoice: RPS, playerTwoChoice: RPS): boolean {
  if (
    (playerOneChoice === RPS.ROCK && playerTwoChoice === RPS.SCISSORS) ||
    (playerOneChoice === RPS.PAPER && playerTwoChoice === RPS.ROCK) ||
    (playerOneChoice === RPS.SCISSORS && playerTwoChoice === RPS.PAPER)
  ) {
    return true;
  }

  return false;
}

function getRoundWinnerAndUpdateGameState(
  game: RockPaperScissorsGameState
): RockPaperScissorsGameState {
  const { playerOne, playerTwo } = game;

  if (playerOne.choice === null || playerTwo.choice === null) {
    throw new Error("Player choice is null");
  }

  // Draw
  if (playerOne.choice === playerTwo.choice) {
    return {
      playerOne: {
        choice: null,
        score: playerOne.score + RESULT_VALUES.DRAW + playerOne.choice,
      },
      playerTwo: {
        choice: null,
        score: playerTwo.score + RESULT_VALUES.DRAW + playerTwo.choice,
      },
    };
  }

  const playerOneWinsRound = playerOneWins(playerOne.choice, playerTwo.choice);

  // draws have been filtered out. So, if player one doesn't win, player two must win
  if (playerOneWinsRound) {
    return {
      playerOne: {
        choice: null,
        score: playerOne.score + RESULT_VALUES.WIN + playerOne.choice,
      },
      playerTwo: {
        choice: null,
        score: playerTwo.score + RESULT_VALUES.LOSE + playerTwo.choice,
      },
    };
  }

  return {
    playerOne: {
      choice: null,
      score: playerOne.score + RESULT_VALUES.LOSE + playerOne.choice,
    },
    playerTwo: {
      choice: null,
      score: playerTwo.score + RESULT_VALUES.WIN + playerTwo.choice,
    },
  };
}

export function getPartOneAnswer(input: string): RockPaperScissorsGameState {
  const lines = input.split("\n");
  let game: RockPaperScissorsGameState = {
    playerOne: getInitialPlayerState(),
    playerTwo: getInitialPlayerState(),
  };

  for (const line of lines) {
    const [playerOneChoice, playerTwoChoice] = line.split(" ");

    game.playerOne.choice = getPlayerChoiceValue(playerOneChoice);
    game.playerTwo.choice = getPlayerChoiceValue(playerTwoChoice);
    console.log(game);

    game = getRoundWinnerAndUpdateGameState(game);
  }

  return game;
}
