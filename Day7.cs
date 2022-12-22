using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Runtime.CompilerServices;

namespace Aoc2022
{
    public class Day7
    {
        public static readonly uint MAX_CAPACITY       = 70000000;
        public static readonly uint UPDATE_REQUIREMENT = 30000000;
        public static uint FREE_SPACE;
        public static KeyValuePair<string, uint> CHOSEN_DIRECTORY = new KeyValuePair<string, uint>("", UPDATE_REQUIREMENT);

        public static void Main(string[] args)
        {
            Directory root = new Directory("/");
            Guid rootPtr = root.GetRefId();
            Guid currentPtr = rootPtr;

            Console.WriteLine("Root Directory GUID: " + rootPtr.ToString());


            Console.WriteLine("Hello, world!");
            List<string> history = new();

            using (StreamReader sr = new StreamReader(new FileStream(Environment.CurrentDirectory + "/day7.txt", FileMode.Open)))
            {
                while (!sr.EndOfStream)
                    history.Add(sr.ReadLine());
            }

            int position = 0;
            while (position < history.Count)
            {
                string[] tokens = history[position].Split(' ');
                switch(tokens[0])
                {
                    case "$":
                        if (tokens[1] == "cd" && tokens[2] == "/")
                        {
                            currentPtr = rootPtr;
                            break;
                        }
                        else if (tokens[1] == "cd" && tokens[2] != "..")
                        {
                            Directory newDir = new Directory(tokens[2]);
                            newDir.Parent = currentPtr;
                            Extensions.GetObjectByRef<Directory>(currentPtr).Subdirectories.Add(newDir);
                            currentPtr = newDir.GetRefId();
                            break;
                        }
                        else if (tokens[1] == "cd" && tokens[2] == "..")
                        {
                            currentPtr = Extensions.GetObjectByRef<Directory>(currentPtr).Parent;
                            break;
                        }
                        else if (tokens[1] == "ls")
                        {
                            break;
                        }
                        break;
                    default:
                        if (tokens[0] == "dir")
                            break;
                        else if (uint.TryParse(tokens[0], out uint size))
                        {
                            Directory cur = Extensions.GetObjectByRef<Directory>(currentPtr);
                            cur.Files.Add(new() { Size = size, Name = tokens[1] });
                        }
                        break;

                }
                position++;
            }

            uint totalSize = root.GetSize();
            Day7.FREE_SPACE = MAX_CAPACITY - totalSize;

            Console.WriteLine("Printing Directory...");
            root.PrettyPrint();
        }
    }

    public class Directory
    {
        public string Name { get; set; }
        public List<Directory> Subdirectories { get; set; }
        public List<File> Files { get; set; }
        public Guid Parent { get; set; }

        public Directory(string name = "")
        {
            Name = name;
            Subdirectories = new List<Directory>();
            Files = new List<File>();
            Parent = Guid.Empty;
        }

        public uint GetSize()
        {
            uint totalSize = 0;

            foreach (File f in Files)
                totalSize += f.Size;

            foreach (Directory dir in Subdirectories)
                totalSize += dir.GetSize();

            return totalSize;
        }

        public void PrettyPrint(int level = 1)
        {
            uint dirSize = GetSize();

            for (int i = 0; i < level; i++)
                Console.Write("-");
            Console.WriteLine(" " + Name + " (dir, size=" + dirSize + ")");

            if (Day7.FREE_SPACE + dirSize >= Day7.UPDATE_REQUIREMENT && dirSize < Day7.CHOSEN_DIRECTORY.Value)
            {
                Console.WriteLine("***");
                Day7.CHOSEN_DIRECTORY = new KeyValuePair<string, uint>(Name, dirSize);
            }

            foreach (Directory d in Subdirectories)
                d.PrettyPrint(level + 1);

            //foreach (File f in Files)
            //{
            //    for (int i = 0; i < level; i++)
            //        Console.Write("-");
            //    Console.WriteLine(" " + f.Name + " (file, size=" + f.Size + ")");
            //}
        }
    }

    public class File
    {
        public string Name { get; set; }
        public uint Size { get; set; }

        public File()
        {
            Name = string.Empty;
            Size = 0;
        }
    }

    public static class Extensions
    {
        private static readonly ConditionalWeakTable<object, RefId> _ids = new ConditionalWeakTable<object, RefId>();

        public static Guid GetRefId<T>(this T obj) where T : class
        {
            if (obj == null)
                return default;

            return _ids.GetOrCreateValue(obj).Id;
        }

        public static T GetObjectByRef<T>(Guid valid) where T: class
        {
            return _ids.Where(x => x.Value.Id == valid).First().Key as T;
        }

        private class RefId
        {
            public Guid Id { get; } = Guid.NewGuid();
        }
    }
}
