/// Game.TestColorHexToRgba("AABBCCDD");
public static exec func TestColorHexToRgba(game: GameInstance, hex: String) -> Void {
    let test = ColorHexToRgba(hex);
    LogChannel(n"DEBUG", ToString(test));
}