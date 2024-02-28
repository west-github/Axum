import os


# PGPASSWORD=your_password psql -U your_user -d your_database -h your_host -p your_port -W -c "SELECT * FROM your_table;"
def pexec(user: str, port: int, pwd: str | int, cmd: str) -> None:
    _cmd = f"SET PGPASSWORD={pwd} psql -U {user} -p {port} -w"

    if cmd:
        _cmd += f" -f {cmd}"

    print(_cmd)
    os.system(_cmd)


def rfdir(path: str) -> list[str]:
    files = [f for f in os.listdir(path) if os.path.isfile(os.path.join(path, f))]

    return sorted(files)


def psqlF(path: str, mode: str | None = None) -> list[str]:
    files = [
        file for file in rfdir(path) if (mode == "rc") == (file == "00-recreate-db.sql")
    ]

    return [os.path.join(path, file) for file in files]


def web_server(path: str | None = None) -> None:
    path = path if path else input("Enter Sql Dir (exit to close )>>> ")

    # Recreate Dbs
    if True:
        for file in psqlF(path=path, mode="rc"):
            pexec("postgres", 5433, 1234, file)

    # Create Others
    for file in psqlF(path):
        pexec("web", 5433, "web", file)

    print("Database set successfully")

    bin_cmd = input("Binary Command to run >>>")

    os.system(bin_cmd)


def api_server() -> None:
    pass


if __name__ == "__main__":
    print("Setup - Development Setup Program")

    opt = ["Web Server", "Api Setup", "Exit"]

    for i, v in enumerate(opt):
        print(f"{i + 1}: {v}")

    try:
        _input = int(input(f"Select (1 - {len(opt)}) to continue >> "))

        ops = {1: lambda: web_server("./core/sql/web-server"), 2: api_server}

        if _input == 3:
            os.system("cls")

        ops[_input]()

    finally:
        # os.system("cls")
        ...
# ./core/sql/web-server
